use crate::hittable::Hittable;

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
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let mut hit_record = None;
        let mut closest_t = t_max;
        for i in &self.list {
            if let Some(hit) = i.hit(ray, t_min, closest_t) {
                closest_t = hit.t;
                hit_record = Some(hit);
            }
        }
        return hit_record;
    }
}
