extern crate rand;

use vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::zero();

    loop {
        point = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);

        if point.squared_length() < 1.0 {
            break;
        }
    }

    point
}