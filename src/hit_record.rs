use vec3::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
        }
    }

    pub fn copy_from(&mut self, rhs:HitRecord) {
        self.point = rhs.point;
        self.normal = rhs.normal;
        self.t = rhs.t;
    }
}