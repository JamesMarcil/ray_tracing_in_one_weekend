use vec3::Vec3;
use ray::Ray;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from:Vec3, look_at:Vec3, up:Vec3, vertical_fov:f32, aspect_ratio:f32) -> Camera {
        let theta = vertical_fov * (PI / 180.0);
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(up, w).unit();
        let v = Vec3::cross(w, u);

        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
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
