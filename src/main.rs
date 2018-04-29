mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn get_color(r:Ray) -> Vec3 {
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);

    Vec3::make(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::make(0.5, 0.7, 1.0) * t
}

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3::make(-2.0, -1.0, -1.0);
    let horizontal = Vec3::make(4.0, 0.0, 0.0);
    let vertical = Vec3::make(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::make(origin, lower_left_corner + horizontal * u + vertical * v);

            let color = get_color(r);

            let ir = (255.99 * color.r()) as u8;
            let ig = (255.99 * color.g()) as u8;
            let ib = (255.99 * color.b()) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
