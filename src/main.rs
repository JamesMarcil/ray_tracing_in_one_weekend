fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
