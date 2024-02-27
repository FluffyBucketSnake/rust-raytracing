use crate::{hittable::Hittable, interval::Interval, prelude::*};

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, mut t_interval: Interval) -> Option<crate::hittable::HitRecord> {
        let mut hit_record = None;
        for i in &self.list {
            if let Some(hit) = i.hit(ray, t_interval) {
                t_interval = t_interval.with_max(hit.t);
                hit_record = Some(hit);
            }
        }
        return hit_record;
    }
}
