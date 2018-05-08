use vec3::Vec3;
use material::Material;

pub struct HitRecord<'material> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'material Material,
}

impl<'material> HitRecord<'material> {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f32,
        material: &'material Material,
    ) -> HitRecord<'material> {
        HitRecord {
            point,
            normal,
            t,
            material,
        }
    }
}
