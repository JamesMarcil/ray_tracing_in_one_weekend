use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use hit_record::HitRecord;
use material::Material;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<Material + Send + Sync>) -> Sphere {
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

impl Hitable for Sphere {
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
                let point = r.point_at_parameter(root_one);
                let normal = (point - self.center) / self.radius;
                let t = root_one;
                let material = &*self.material;
                let hit_record = HitRecord::new(point, normal, t, material);
                return Some(hit_record);
            }

            let root_two = (-b + f32::sqrt(b * b - a * c)) / a;
            if root_two < t_max && root_two > t_min {
                let point = r.point_at_parameter(root_two);
                let normal = (point - self.center) / self.radius;
                let t = root_two;
                let material = &*self.material;
                let hit_record = HitRecord::new(point, normal, t, material);
                return Some(hit_record);
            }
        }

        None
    }
}
