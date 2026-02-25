use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

/// 複数の Hittable オブジェクトをまとめて管理するリスト。
/// トレイトオブジェクト (Box<dyn Hittable>) で異なる型を混在させられる。
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    /// オブジェクトをリストに追加する。
    /// `impl Hittable + 'static` を受け取り Box に包む。
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    /// リスト内の全オブジェクトにレイを当て、最も近い交点を返す。
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // 現時点で見つかった最近傍の t。これを上限として更新していく。
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        for object in &self.objects {
            // 有効範囲の上限を closest_so_far に絞ることで、
            // 既に見つかった交点より遠い解は自動的に無視される
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
