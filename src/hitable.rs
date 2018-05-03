use ray::Ray;
use hit_record::HitRecord;
use material::Material;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
