use vec3::Vec3;
use material::Material;

pub struct HitRecord<'material> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Option<&'material Material>,
}

impl<'material> HitRecord<'material> {
    pub fn new() -> HitRecord<'material> {
        HitRecord {
            point: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            material: None,
        }
    }

    pub fn copy_from(&mut self, rhs: HitRecord<'material>) {
        self.point = rhs.point;
        self.normal = rhs.normal;
        self.t = rhs.t;
        self.material = rhs.material;
    }
}
