use ray::Ray;
use math::{random_in_unit_sphere, shlick};
use vec3::Vec3;
use hit_record::HitRecord;
use rand::random;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray);
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();

        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.albedo;

        (true, attenuation, scattered)
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: f32::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected = Vec3::reflect(ray_in.direction().unit(), hit_record.normal);

        let scattered = Ray::new(
            hit_record.point,
            reflected + (self.fuzz * random_in_unit_sphere()),
        );

        let attenuation = self.albedo;

        let did_scatter = Vec3::dot(scattered.direction(), hit_record.normal) > 0.0;

        (did_scatter, attenuation, scattered)
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let cosine;
        let ni_over_nt;
        let outward_normal;
        if Vec3::dot(ray_in.direction(), hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3::dot(ray_in.direction(), hit_record.normal)
                / ray_in.direction().length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine =
                -Vec3::dot(ray_in.direction(), hit_record.normal) / ray_in.direction().length();
        }

        let scattered;
        let reflected = Vec3::reflect(ray_in.direction(), hit_record.normal);

        match Vec3::refract(ray_in.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => {
                let random_value = random::<f32>();
                let reflect_probability = shlick(cosine, self.ref_idx);

                if random_value < reflect_probability {
                    scattered = Ray::new(hit_record.point, reflected);
                } else {
                    scattered = Ray::new(hit_record.point, refracted);
                }
            }
            None => {
                scattered = Ray::new(hit_record.point, reflected);
            }
        }

        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        return (true, attenuation, scattered);
    }
}
