extern crate image;
extern crate rand;
extern crate rayon;

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
use hitable_list::HitableList;
use sphere::Sphere;
use material::{Dielectric, Lambertian, Metal};
use rayon::prelude::*;

fn get_color(r: Ray, hitable: &Hitable, depth: i32) -> Vec3 {
    match hitable.hit(&r, 0.001, std::f32::MAX) {
        Some(hit_record) => {
            let (did_scatter, attenuation, scattered) =
                hit_record.material.scatter(&r, &hit_record);
            if depth < 50 && did_scatter {
                return get_color(scattered, hitable, depth + 1) * attenuation;
            }
            return Vec3::zero();
        }
        None => {
            let unit_direction = r.direction().unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() {
    let mut elements: Vec<Box<Hitable + Sync>> = vec![];

    let material_one = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));

    let sphere_one = Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_one,
    ));

    elements.push(sphere_one);

    for a in -11..11 {
        for b in -11..11 {
            let material_probability = rand::random::<f32>();

            let center = Vec3::new(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // Diffuse
                if material_probability < 0.8 {
                    let material = Box::new(Lambertian::new(Vec3::new(
                        rand::random::<f32>() * rand::random::<f32>(),
                        rand::random::<f32>() * rand::random::<f32>(),
                        rand::random::<f32>() * rand::random::<f32>(),
                    )));

                    let sphere = Box::new(Sphere::new(center, 0.2, material));

                    elements.push(sphere);
                // Metal
                } else if material_probability < 0.95 {
                    let material = Box::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                        ),
                        0.5 * rand::random::<f32>(),
                    ));

                    let sphere = Box::new(Sphere::new(center, 0.2, material));

                    elements.push(sphere);
                // Glass
                } else {
                    let material = Box::new(Dielectric::new(1.5));

                    let sphere = Box::new(Sphere::new(center, 0.2, material));

                    elements.push(sphere);
                }
            }
        }
    }

    let material_two = Box::new(Dielectric::new(1.5));
    let sphere_two = Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_two));
    elements.push(sphere_two);

    let material_three = Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let sphere_three = Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_three));
    elements.push(sphere_three);

    let material_four = Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    let sphere_four = Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_four));
    elements.push(sphere_four);

    let world = HitableList::new(elements);

    let nx = 1600;
    let ny = 900;
    let num_samples = 10;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let vertical_fov = 20.0;
    let aspect_ratio = nx as f32 / ny as f32;
    let focus_distance = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        vertical_fov,
        aspect_ratio,
        aperture,
        focus_distance,
    );

    let mut pixels = vec![[0, 0, 0]; nx * ny];

    pixels.par_iter_mut().enumerate().for_each(|(i, value)| {
        let mut color = Vec3::zero();

        let x = i % nx;
        let y = ny - (i / nx);

        for _ in 0..num_samples {
            let u = (x as f32 + rand::random::<f32>()) / nx as f32;
            let v = (y as f32 + rand::random::<f32>()) / ny as f32;

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

        *value = [ir, ig, ib];
    });

    let result = pixels.iter().fold(Vec::new(), |mut array, value| {
        array.push(value[0]);
        array.push(value[1]);
        array.push(value[2]);
        array
    });

    image::save_buffer("out.png", &result, nx as u32, ny as u32, image::ColorType::RGB(8))
        .expect("Failed to write file!");
}
