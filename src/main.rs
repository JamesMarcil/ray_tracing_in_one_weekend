mod vec3;

use vec3::Vec3;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = Vec3::make(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);

            let ir = (255.99 * v.r()) as u8;
            let ig = (255.99 * v.g()) as u8;
            let ib = (255.99 * v.b()) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
