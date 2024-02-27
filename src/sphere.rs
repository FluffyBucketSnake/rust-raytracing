use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center;
        let a = ray.direction().lenght_squared();
        let half_b = center_to_origin.dot(&ray.direction());
        let c = center_to_origin.lenght_squared() - self.radius * self.radius;

        let delta = half_b * half_b - a * c;
        if delta < 0.0 {
            return None;
        }
        let delta_sqrt = delta.sqrt();

        let t_range = t_min..=t_max;
        let mut t = (-half_b - delta_sqrt) / a;
        if !t_range.contains(&t) {
            t = (-half_b + delta_sqrt) / a;
            if !t_range.contains(&t) {
                return None;
            }
        }

        let position = ray.at(t);
        let outward_normal = (position - self.center) / self.radius;

        return Some(HitRecord::from_outward_normal(
            t,
            position,
            outward_normal,
            &ray,
        ));
    }
}
