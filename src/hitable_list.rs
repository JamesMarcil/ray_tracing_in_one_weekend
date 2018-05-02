use ray::Ray;
use hit_record::HitRecord;
use hitable::Hitable;

pub struct HitableList<'s> {
    elements: Vec<&'s Hitable>,
}

impl<'s> HitableList<'s> {
    pub fn new(elements: Vec<&'s Hitable>) -> HitableList<'s> {
        HitableList { elements }
    }
}

impl<'s> Hitable for HitableList<'s> {
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
