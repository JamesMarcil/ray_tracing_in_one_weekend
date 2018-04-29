use vec3::Vec3;
use ray::Ray;

#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Camera {
        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn lower_left_corner(&self) -> Vec3 {
        self.lower_left_corner
    }

    pub fn horizontal(&self) -> Vec3 {
        self.horizontal
    }

    pub fn vertical(&self) -> Vec3 {
        self.vertical
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
