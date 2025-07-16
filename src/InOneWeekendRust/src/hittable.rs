pub mod ray;
pub strcut HitRecord{
    pub p: glam::Vec3;
    pub normal: glam::Vec3;
    pub t: f32;
}
pub trait Hittable{
    hit(ray: &ray::Ray, ray_tmin: f32, ray_tmax: f32)->Option<HitRecord>;
}