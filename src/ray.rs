use vec3::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray::make(Vec3::zero(), Vec3::zero())
    }

    pub fn make(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point_at_parameter(&self, t:f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
