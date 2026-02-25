use crate::interval::Interval;
use crate::ray::Ray;

/// レイがオブジェクトに当たったときの情報
pub struct HitRecord {
    /// 交点の座標
    pub p: glam::Vec3,
    /// 交点における法線（常にレイと逆向きに補正済み）
    pub normal: glam::Vec3,
    /// レイのパラメータ t（r.at(t) = p）
    pub t: f32,
    /// レイがオブジェクトの表面（外側）から当たったか
    pub front_face: bool,
}

impl HitRecord {
    /// 法線の向きをレイと逆向きに揃える。
    /// outward_normal は常に外向きの単位法線。
    /// レイが外側から来た場合はそのまま、内側から来た場合は反転する。
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: glam::Vec3) {
        // 内積が負 → レイと法線が逆向き → レイは外側から入射
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

/// レイとの交差判定を持つオブジェクトのトレイト
pub trait Hittable {
    /// レイ r が区間 ray_t 内で交差するか判定する。
    /// 交差した場合は Some(HitRecord)、しない場合は None を返す。
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
