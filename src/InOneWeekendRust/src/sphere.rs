use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct Sphere {
    pub center: glam::Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: glam::Vec3, radius: f32) -> Self {
        Sphere {
            center,
            // 負の半径は意味を持たないので 0 以上に補正する
            radius: radius.max(0.0),
        }
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
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord {
            p,
            normal: glam::Vec3::ZERO,
            t: root,
            front_face: false,
        };
        // レイの進行方向に応じて法線を表側に向ける
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
