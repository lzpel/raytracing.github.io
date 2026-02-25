mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;

use std::sync::Arc;

use camera::Camera;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;

fn main() {
    // ─── マテリアル定義 ──────────────────────────────────────────────────
    // 地面: 黄緑の Lambertian
    let mat_ground = Arc::new(Lambertian {
        albedo: glam::Vec3::new(0.8, 0.8, 0.0),
    });
    // 中央球: 青みがかった Lambertian
    let mat_center = Arc::new(Lambertian {
        albedo: glam::Vec3::new(0.1, 0.2, 0.5),
    });
    // 左球（外側）: ガラス（屈折率 1.5）
    let mat_left = Arc::new(Dielectric {
        refraction_index: 1.50,
    });
    // 左球（内側バブル）: 屈折率を 1/1.5 にすることで
    // ガラス→空気界面を再現し、中空ガラスの空気層を表現する（11.5節）
    let mat_bubble = Arc::new(Dielectric {
        refraction_index: 1.00 / 1.50,
    });
    // 右球: ゴールドっぽい Metal（fuzz=1.0 で少しぼかす）
    let mat_right = Arc::new(Metal::new(glam::Vec3::new(0.8, 0.6, 0.2), 1.0));

    // ─── ワールド構築 ────────────────────────────────────────────────────
    let mut world = HittableList::new();

    // 地面（大きな球）
    world.add(Sphere::new(
        glam::Vec3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    ));
    // 中央球（Lambertian）
    world.add(Sphere::new(
        glam::Vec3::new(0.0, 0.0, -1.2),
        0.5,
        mat_center,
    ));
    // 左球・外側（ガラス、正の半径 → 外側面）
    world.add(Sphere::new(
        glam::Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    ));
    // 左球・内側バブル（負の半径 → 法線が内向きになり空気層の内面を形成）
    world.add(Sphere::new(
        glam::Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        mat_bubble,
    ));
    // 右球（Metal）
    world.add(Sphere::new(
        glam::Vec3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    ));

    // ─── カメラ設定・レンダリング ─────────────────────────────────────────
    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    let img = cam.render(&world);
    img.save("../../image1.png").unwrap();
}
