use crate::color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use image::RgbImage;
use rand::Rng;

pub struct Camera {
    /// アスペクト比（幅 / 高さ）
    pub aspect_ratio: f32,
    /// 出力画像の横ピクセル数
    pub image_width: u32,
    /// 1ピクセルあたりのサンプル数（アンチエイリアシング用）
    pub samples_per_pixel: u32,
    /// レイの最大反射回数（これを超えたら黒を返す）
    pub max_depth: u32,
}

impl Camera {
    /// ワールドをレンダリングして RgbImage を返す
    pub fn render(&self, world: &dyn Hittable) -> RgbImage {
        // アスペクト比から縦ピクセル数を計算（最低 1 ピクセル）
        let image_height = ((self.image_width as f32 / self.aspect_ratio) as u32).max(1);
        // 複数サンプルの平均を取るためのスケール係数
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        // カメラ位置（原点）
        let camera_center = glam::Vec3::ZERO;
        let focal_length = 1.0_f32; // カメラ中心からビューポートまでの距離
        let viewport_height = 2.0_f32;
        // 実際のピクセル比からビューポート幅を決める（aspect_ratio そのままでは誤差が出るため）
        let viewport_width =
            viewport_height * (self.image_width as f32 / image_height as f32);

        // ビューポートの水平・垂直エッジベクトル
        // viewport_v は下向き（画像座標系の y 軸と合わせるため負）
        let viewport_u = glam::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = glam::Vec3::new(0.0, -viewport_height, 0.0);

        // ピクセル間の移動量（1ピクセル分のデルタ）
        let pixel_delta_u = viewport_u / self.image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // ビューポート左上の座標、そこからピクセル(0,0)の中心位置を求める
        let viewport_upper_left = camera_center
            - glam::Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let mut img = RgbImage::new(self.image_width, image_height);

        for j in 0..image_height {
            eprintln!("Scanlines remaining: {}", image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = glam::Vec3::ZERO;
                // samples_per_pixel 本のランダムなレイを飛ばして平均を取る（アンチエイリアシング）
                for _ in 0..self.samples_per_pixel {
                    let r = get_ray(
                        i,
                        j,
                        camera_center,
                        pixel00_loc,
                        pixel_delta_u,
                        pixel_delta_v,
                    );
                    pixel_color += ray_color(&r, self.max_depth, world);
                }
                // サンプルの平均をガンマ補正してピクセルに書き込む
                let rgb = color::to_rgb(pixel_samples_scale * pixel_color);
                img.put_pixel(i, j, rgb);
            }
        }

        eprintln!("Done.");
        img
    }
}

/// ピクセル (i, j) に対応するレイを生成する。
/// ピクセル中心の近傍をランダムにサンプリングしてアンチエイリアシングを実現する。
fn get_ray(
    i: u32,
    j: u32,
    camera_center: glam::Vec3,
    pixel00_loc: glam::Vec3,
    pixel_delta_u: glam::Vec3,
    pixel_delta_v: glam::Vec3,
) -> Ray {
    // ピクセル中心から ±0.5 の範囲でランダムにオフセットを加える
    let offset = sample_square();
    let pixel_sample = pixel00_loc
        + ((i as f32 + offset.x) * pixel_delta_u)
        + ((j as f32 + offset.y) * pixel_delta_v);
    let ray_direction = pixel_sample - camera_center;
    Ray {
        orig: camera_center,
        dir: ray_direction,
    }
}

/// [-0.5, 0.5) × [-0.5, 0.5) の範囲でランダムな 2D オフセットを返す
fn sample_square() -> glam::Vec2 {
    let mut rng = rand::thread_rng();
    glam::Vec2::new(
        rng.gen_range(0.0_f32..1.0) - 0.5,
        rng.gen_range(0.0_f32..1.0) - 0.5,
    )
}

/// レイの色を再帰的に計算する。
/// オブジェクトに当たった場合は拡散反射、当たらなければ背景グラデーションを返す。
fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> glam::Vec3 {
    // 反射回数の上限に達したら光の寄与なし（黒）
    if depth == 0 {
        return glam::Vec3::ZERO;
    }

    // t_min = 0.001: 浮動小数点誤差で自己交差（シャドウアクネ）が起きないよう
    // 交点のごく近傍を無視する（section 9.3）
    if let Some(rec) = world.hit(r, Interval::new(0.001, f32::INFINITY)) {
        // True Lambertian 拡散反射（section 9.4）:
        // 法線方向にランダム単位ベクトルを加えた方向に散乱する
        let mut direction = rec.normal + random_unit_vector();
        // 散乱方向がほぼゼロベクトルになる縮退ケースを防ぐ
        if near_zero(direction) {
            direction = rec.normal;
        }
        let scattered = Ray {
            orig: rec.p,
            dir: direction,
        };
        // 反射のたびに 50% 減衰させて再帰
        return 0.5 * ray_color(&scattered, depth - 1, world);
    }

    // 何にも当たらなかった場合: 空の青〜白グラデーション（背景）
    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0); // y が高いほど青み
    (1.0 - a) * glam::Vec3::new(1.0, 1.0, 1.0) + a * glam::Vec3::new(0.5, 0.7, 1.0)
}

/// 単位球内のランダムな単位ベクトルを棄却サンプリングで生成する。
/// [-1,1]³ の立方体内に一様乱数を生成し、単位球の内側に収まるまで繰り返す。
fn random_unit_vector() -> glam::Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = glam::Vec3::new(
            rng.gen_range(-1.0_f32..1.0),
            rng.gen_range(-1.0_f32..1.0),
            rng.gen_range(-1.0_f32..1.0),
        );
        let lensq = p.length_squared();
        // lensq が極端に小さい場合は正規化時にオーバーフローする恐れがあるため除外
        if 1e-160_f32 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt(); // 正規化して単位ベクトルにする
        }
    }
}

/// ベクトルの全成分が極めて小さいか判定する（縮退方向の検出用）
fn near_zero(v: glam::Vec3) -> bool {
    let s = 1e-8_f32;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}
