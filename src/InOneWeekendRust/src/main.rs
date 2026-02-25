mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;

fn main() {
    // ワールドに球を2つ配置する:
    // - 小さな球（前景）: 中心 (0, 0, -1)、半径 0.5
    // - 大きな球（地面）: 中心 (0, -100.5, -1)、半径 100
    let mut world = HittableList::new();
    world.add(Sphere::new(glam::Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(glam::Vec3::new(0.0, -100.5, -1.0), 100.0));

    // カメラ設定
    let cam = Camera {
        aspect_ratio: 16.0 / 9.0, // 16:9 ワイドスクリーン
        image_width: 400,
        samples_per_pixel: 100, // アンチエイリアシング: 1ピクセルあたり100サンプル
        max_depth: 50,          // 拡散反射の最大再帰深度
    };

    let img = cam.render(&world);
    img.save("../../image1.png").unwrap();
}
