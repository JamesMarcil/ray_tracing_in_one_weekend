use rand::random;
use vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::zero();

    loop {
        point = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);

        if point.squared_length() < 1.0 {
            break;
        }
    }

    point
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut point = Vec3::zero();

    loop {
        point = Vec3::new(random::<f32>(), random::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);

        if point.squared_length() < 1.0 {
            break;
        }
    }

    point
}

pub fn shlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);

    r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
}
