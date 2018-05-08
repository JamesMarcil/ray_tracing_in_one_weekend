use vec3::Vec3;
use ray::Ray;
use math::random_in_unit_disk;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,

    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = vertical_fov * (PI / 180.0);
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(up, w).unit();
        let v = Vec3::cross(w, u);

        Camera {
            lower_left_corner: look_from - half_width * focus_distance * u
                - half_height * focus_distance * v
                - focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            origin: look_from,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
