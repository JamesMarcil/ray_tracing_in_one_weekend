extern crate rand;

mod math;
mod camera;
mod vec3;
mod ray;
mod hitable;
mod hit_record;
mod hitable_list;
mod sphere;

use camera::Camera;
use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use hit_record::HitRecord;
use hitable_list::HitableList;
use sphere::Sphere;

fn get_color(r: Ray, hitable: &Hitable) -> Vec3 {
    match hitable.hit(&r, 0.001, std::f32::MAX) {
        Some(hit_record) => {
            let target = hit_record.point + hit_record.normal + math::random_in_unit_sphere();
            return get_color(
                Ray::new(hit_record.point, target - hit_record.point),
                hitable,
            ) * 0.5;
        }
        None => {
            let unit_direction = r.direction().unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    let num_samples: i32 = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let camera = Camera::new(
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::zero(),
    );

    let sphere_one = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_two = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);

    let elements = vec![&sphere_one as &Hitable, &sphere_two as &Hitable];

    let world = HitableList::new(elements);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::zero();

            for _ in 0..num_samples {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;

                let r = camera.get_ray(u, v);

                color += get_color(r, &world);
            }

            color /= num_samples as f32;

            // Appromixate gamma correction
            color = Vec3::new(
                f32::sqrt(color.r()),
                f32::sqrt(color.g()),
                f32::sqrt(color.b()),
            );

            let ir = (255.99 * color.r()) as u8;
            let ig = (255.99 * color.g()) as u8;
            let ib = (255.99 * color.b()) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
