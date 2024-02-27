mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod prelude;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use std::error::Error;
use vec3::{Point3, Vec3};

type Color = Vec3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::from_aspect_ratio(image_width, aspect_ratio);

    camera.render(&world)?;

    Ok(())
}
