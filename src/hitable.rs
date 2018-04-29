use vec3::Vec3;
use ray::Ray;
use hit_record::HitRecord;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}
