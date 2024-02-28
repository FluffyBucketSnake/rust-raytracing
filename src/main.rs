mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod prelude;
mod random;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use prelude::*;
use sphere::Sphere;
use std::{error::Error, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let camera = Camera::from_aspect_ratio(image_width, aspect_ratio, samples_per_pixel, max_depth);

    camera.render(&world)?;

    Ok(())
}
