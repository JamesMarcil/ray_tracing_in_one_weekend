use ray::Ray;
use hit_record::HitRecord;
use hitable::Hitable;

pub struct HitableList<'s> {
    elements: Vec<&'s Hitable>
}

impl<'s> HitableList<'s> {
    pub fn new(elements:Vec<&'s Hitable>) -> HitableList<'s> {
        HitableList{elements}
    }
}

impl<'s> Hitable for HitableList<'s> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_hit_record = HitRecord::new();
        let mut did_hit_anything = false;
        let mut closest_so_far = t_max;

        for element in &self.elements {
            if element.hit(r, t_min, closest_so_far, &mut temp_hit_record) {
                did_hit_anything = true;
                closest_so_far = temp_hit_record.t;
                hit_record.copy_from(temp_hit_record);
            }
        }

        did_hit_anything
    }
}