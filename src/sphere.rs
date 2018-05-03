use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use hit_record::HitRecord;
use material::{HasMaterial, Material};

pub struct Sphere<'material> {
    center: Vec3,
    radius: f32,
    material: &'material Material,
}

impl<'material> Sphere<'material> {
    pub fn new(center: Vec3, radius: f32, material: &'material Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl<'material> Hitable for Sphere<'material> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;

        let direction = r.direction();

        let a = Vec3::dot(direction, direction);
        let b = Vec3::dot(oc, direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let root_one = (-b - f32::sqrt(b * b - a * c)) / a;
            if root_one < t_max && root_one > t_min {
                let mut hit_record = HitRecord::new();
                hit_record.t = root_one;
                hit_record.point = r.point_at_parameter(root_one);
                hit_record.normal = (hit_record.point - self.center) / self.radius;
                hit_record.material = Some(self.material);
                return Some(hit_record);
            }

            let root_two = (-b + f32::sqrt(b * b - a * c)) / a;
            if root_two < t_max && root_two > t_min {
                let mut hit_record = HitRecord::new();
                hit_record.t = root_two;
                hit_record.point = r.point_at_parameter(root_two);
                hit_record.normal = (hit_record.point - self.center) / self.radius;
                hit_record.material = Some(self.material);
                return Some(hit_record);
            }
        }

        None
    }
}

impl<'material> HasMaterial for Sphere<'material> {
    fn material<'s>(&'s self) -> &'s Material {
        self.material
    }
}
