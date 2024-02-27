use crate::{
    prelude::*,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub struct HitRecord {
    pub t: float,
    pub position: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from_outward_normal(
        t: float,
        position: Point3,
        outward_normal: Vec3,
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        Self {
            t,
            position,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: float, t_max: float) -> Option<HitRecord>;
}
