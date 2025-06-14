use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Debug)]
pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        HittableList { list: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.list.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                rec.set_t(temp_rec.t());
                rec.set_p(temp_rec.p());
                rec.set_normal(temp_rec.normal());
            }
        }
        hit_anything
    }
}
