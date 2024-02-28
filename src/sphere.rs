use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    prelude::*,
};

pub struct Sphere {
    center: Point3,
    radius: float,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: float, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center;
        let a = ray.direction().lenght_squared();
        let half_b = center_to_origin.dot(&ray.direction());
        let c = center_to_origin.lenght_squared() - self.radius * self.radius;

        let delta = half_b * half_b - a * c;
        let delta_sqrt = (delta >= 0.0).then(|| delta.sqrt())?;

        let t = t_interval
            .surrounds_some((-half_b - delta_sqrt) / a)
            .or_else(|| t_interval.surrounds_some((-half_b + delta_sqrt) / a))?;

        let position = ray.at(t);
        let outward_normal = (position - self.center) / self.radius;

        return Some(HitRecord::from_outward_normal(
            t,
            position,
            self.material.clone(),
            outward_normal,
            &ray,
        ));
    }
}
