use ray::Ray;
use math::random_in_unit_sphere;
use vec3::Vec3;
use hit_record::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray);
}

pub trait HasMaterial {
    fn material<'material>(&'material self) -> &'material Material;
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
