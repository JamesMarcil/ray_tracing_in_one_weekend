use ray::Ray;
use hit_record::HitRecord;
use hitable::Hitable;

pub struct HitableList {
    elements: Vec<Box<Hitable + Sync>>,
}

impl HitableList {
    pub fn new(elements: Vec<Box<Hitable + Sync>>) -> HitableList {
        HitableList { elements }
    }
}

impl Hitable for HitableList {
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
