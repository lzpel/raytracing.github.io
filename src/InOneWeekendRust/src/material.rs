use std::sync::Arc;

use rand::Rng;

use crate::hittable::HitRecord;
use crate::ray::Ray;

/// マテリアルを複数オブジェクトで共有するためのスマートポインタ型エイリアス。
/// C++ の shared_ptr<material> に相当する。
pub type MatPtr = Arc<dyn Material + Send + Sync>;

/// マテリアルのトレイト。レイがサーフェスに当たったときの散乱挙動を定義する。
pub trait Material: Send + Sync {
    /// 入射レイ r_in と交点情報 rec を受け取り、散乱を計算する。
    /// 散乱が起きた場合は Some((attenuation, scattered_ray)) を返す。
    /// 吸収された場合（光が失われた場合）は None を返す。
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(glam::Vec3, Ray)>;
}

// ─── Lambertian（拡散反射）────────────────────────────────────────────────

/// 完全拡散反射（Lambertian）マテリアル。
/// albedo の色を持ち、入射方向に依らず半球全体に均等に散乱する。
pub struct Lambertian {
    /// 表面の色（反射率）。RGB 各成分は [0, 1]
    pub albedo: glam::Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(glam::Vec3, Ray)> {
        // True Lambertian: 法線 + ランダム単位ベクトル方向に散乱
        let mut scatter_direction = rec.normal + random_unit_vector();
        // 散乱方向がほぼゼロになる縮退ケースを法線で置き換える
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        let scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        Some((self.albedo, scattered))
    }
}

// ─── Metal（鏡面反射）────────────────────────────────────────────────────

/// 金属（鏡面反射）マテリアル。
/// fuzz = 0 なら完全鏡面、fuzz が大きいほど反射がぼける。
pub struct Metal {
    /// 表面の色（反射率）
    pub albedo: glam::Vec3,
    /// ファジー係数。[0, 1] にクランプされる。
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: glam::Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(glam::Vec3, Ray)> {
        // 入射レイを法線で鏡面反射させる
        let reflected = reflect(r_in.direction().normalize(), rec.normal);
        // fuzz > 0 のとき、反射方向にランダムオフセットを加えてぼかす
        let reflected = reflected + self.fuzz * random_unit_vector();
        // 反射レイが法線と同じ半球側にある場合のみ散乱（内側に向いたら吸収）
        if reflected.dot(rec.normal) > 0.0 {
            let scattered = Ray {
                orig: rec.p,
                dir: reflected,
            };
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

// ─── Dielectric（誘電体 / 透明ガラス）────────────────────────────────────

/// 誘電体（ガラス・水などの透明素材）マテリアル。
/// 光は屈折と全反射の両方を示す。吸収がないため attenuation は常に白。
pub struct Dielectric {
    /// 屈折率（真空中を 1.0 として、ガラスは約 1.5）
    pub refraction_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(glam::Vec3, Ray)> {
        // ガラスは光を吸収しないので減衰なし
        let attenuation = glam::Vec3::ONE;

        // front_face = true（外側から入射）なら空気→ガラス（比 = 1/n）
        // front_face = false（内側から入射）ならガラス→空気（比 = n）
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // ri * sin_theta > 1.0 のとき Snell の法則を満たす屈折角が存在しない（全反射）
        let cannot_refract = ri * sin_theta > 1.0;

        // 全反射、または Schlick 近似による確率的反射のどちらかなら鏡面反射
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_f32() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };

        let scattered = Ray {
            orig: rec.p,
            dir: direction,
        };
        Some((attenuation, scattered))
    }
}

// ─── 内部ユーティリティ関数 ──────────────────────────────────────────────

/// 鏡面反射ベクトルを計算する。
/// v - 2 * dot(v, n) * n（法線 n は単位ベクトルを仮定）
fn reflect(v: glam::Vec3, n: glam::Vec3) -> glam::Vec3 {
    v - 2.0 * v.dot(n) * n
}

/// Snell の法則による屈折ベクトルを計算する。
/// uv: 入射方向（単位ベクトル）, n: 法線（単位ベクトル）
/// etai_over_etat: 入射側屈折率 / 透過側屈折率
fn refract(uv: glam::Vec3, n: glam::Vec3, etai_over_etat: f32) -> glam::Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    // 屈折ベクトルを法線に垂直な成分と平行な成分に分解して合成する
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

/// Schlick 近似による反射率を計算する。
/// 視線角度が大きい（浅い角度）ほど反射率が高くなる現象を近似する。
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    // r0: 垂直入射時の反射率
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    // 角度依存の反射率
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

/// [0, 1) の一様乱数を生成する
fn random_f32() -> f32 {
    rand::thread_rng().gen_range(0.0_f32..1.0)
}

/// 単位球内のランダムな単位ベクトルを棄却サンプリングで生成する
fn random_unit_vector() -> glam::Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = glam::Vec3::new(
            rng.gen_range(-1.0_f32..1.0),
            rng.gen_range(-1.0_f32..1.0),
            rng.gen_range(-1.0_f32..1.0),
        );
        let lensq = p.length_squared();
        if 1e-160_f32 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

/// ベクトルの全成分が極めて小さいか判定する（縮退方向の検出用）
fn near_zero(v: glam::Vec3) -> bool {
    let s = 1e-8_f32;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}
