use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MatPtr;
use crate::ray::Ray;

pub struct Sphere {
    pub center: glam::Vec3,
    /// 正の半径: 通常の球。
    /// 負の半径: 法線が内側を向くため「中空ガラス」の内側面として使える（11.5節）。
    pub radius: f32,
    /// 球のマテリアル
    pub mat: MatPtr,
}

impl Sphere {
    pub fn new(center: glam::Vec3, radius: f32, mat: MatPtr) -> Self {
        // 負の半径は中空ガラスの内側面に使うため、意図的にそのまま保持する
        Sphere { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // レイと球の交差判定。
        // 球の方程式 |P - C|² = r² に P = ray.at(t) を代入すると
        // a*t² - 2h*t + c = 0 の形になる（h = dot(d, oc) とおいた最適化済み形式）
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc); // b/2 に相当（符号反転済み）
        let c = oc.length_squared() - self.radius * self.radius;

        // 判別式: h² - a*c
        // 負なら実数解なし（レイは球に当たらない）
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // 近い方の解 t を試し、有効範囲外なら遠い方の解を試す
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        // 球面の外向き法線 = (交点 - 中心) / 半径
        // radius が負のとき、この除算で法線の向きが反転する（中空球の内側面の実現）
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord {
            p,
            normal: glam::Vec3::ZERO,
            t: root,
            front_face: false,
            mat: self.mat.clone(),
        };
        // レイの進行方向に応じて法線を表側に向ける
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
