use std::fmt::Debug;

use crate::hittable::HitRecord;
use crate::prelude::*;

pub trait Material: Debug {
    fn scatter(&self, in_ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _in_ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        return Some((
            self.albedo,
            Ray::new(hit_record.position, scatter_direction),
        ));
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: float,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: float) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = in_ray.direction().unit().reflect(&hit_record.normal);
        return Some((
            self.albedo,
            Ray::new(
                hit_record.position,
                reflected + self.fuzz * Vec3::random_unit_vector(),
            ),
        ));
    }
}

#[derive(Debug)]
pub struct Dielectric {
    index_of_refraction: float,
}

impl Dielectric {
    pub fn new(index_of_refraction: float) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = in_ray.direction().unit();
        let cos_theta = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let scatter_direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand_norm() {
                unit_direction.reflect(&hit_record.normal)
            } else {
                unit_direction.refract(&hit_record.normal, refraction_ratio)
            };

        return Some((
            Color::uniform(1.0),
            Ray::new(hit_record.position, scatter_direction),
        ));
    }
}

fn reflectance(cos_theta: float, refraction_ratio: float) -> float {
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
}
