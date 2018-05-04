extern crate rand;

mod math;
mod camera;
mod vec3;
mod ray;
mod hitable;
mod hit_record;
mod hitable_list;
mod sphere;
mod material;

use camera::Camera;
use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use hit_record::HitRecord;
use hitable_list::HitableList;
use sphere::Sphere;
use material::{Dielectric, HasMaterial, Lambertian, Material, Metal};

fn get_color(r: Ray, hitable: &Hitable, depth: i32) -> Vec3 {
    match hitable.hit(&r, 0.001, std::f32::MAX) {
        Some(hit_record) => match hit_record.material {
            Some(material) => {
                let (did_scatter, attenuation, scattered) = material.scatter(&r, &hit_record);
                if depth < 50 && did_scatter {
                    return get_color(scattered, hitable, depth + 1) * attenuation;
                }
                return Vec3::zero();
            }
            None => {
                return Vec3::zero();
            }
        },
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

    let material_one = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_two = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_three = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);
    let material_four = Dielectric::new(1.5);
    let material_five = Dielectric::new(1.5);

    let sphere_one = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &material_one as &Material);
    let sphere_two = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        &material_two as &Material,
    );
    let sphere_three = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &material_three as &Material);
    let sphere_four = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &material_four as &Material);
    let sphere_five = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        &material_five as &Material,
    );

    let elements = vec![
        &sphere_one,
        &sphere_two,
        &sphere_three,
        &sphere_four,
        &sphere_five,
    ];

    let world = HitableList::new(elements);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::zero();

            for _ in 0..num_samples {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;

                let r = camera.get_ray(u, v);

                color += get_color(r, &world, 0);
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
