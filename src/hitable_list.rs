use ray::Ray;
use hit_record::HitRecord;
use hitable::Hitable;
use material::{HasMaterial, Material};

pub struct HitableList<'s, T: 's> {
    elements: Vec<&'s T>,
}

impl<'s, T> HitableList<'s, T>
where
    T: Hitable + HasMaterial,
{
    pub fn new(elements: Vec<&'s T>) -> HitableList<T> {
        HitableList { elements }
    }
}

impl<'s, T> Hitable for HitableList<'s, T>
where
    T: Hitable + HasMaterial,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for element in &self.elements {
            match element.hit(r, t_min, closest_so_far) {
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    result = Some(hit_record);
                }
                None => {}
            }
        }

        result
    }
}
