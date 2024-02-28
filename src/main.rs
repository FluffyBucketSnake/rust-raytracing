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

use camera::CameraBuilder;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use prelude::*;
use sphere::Sphere;
use std::{error::Error, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        -1000.0 * Point3::Y,
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = rand_norm();
            let center = Point3::new(
                (a as float) + 0.9 * rand_norm(),
                0.2,
                (b as float) + 0.9 + rand_norm(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).lenght() <= 0.9 {
                continue;
            }

            let material: Rc<dyn Material> = if material_choice < 0.8 {
                let albedo = Color::random_norm() * Color::random_norm();
                Rc::new(Lambertian::new(albedo))
            } else if material_choice < 0.95 {
                let albedo = Color::random(0.5, 1.0);
                let fuzz = rand(0.0, 0.5);
                Rc::new(Metal::new(albedo, fuzz))
            } else {
                Rc::new(Dielectric::new(1.5))
            };
            world.add(Box::new(Sphere::new(center, 0.2, material)));
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let camera = CameraBuilder::default()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(1200)
        .with_samples_per_pixel(500)
        .with_max_depth(50)
        .with_vfov(20.0)
        .with_look_from(Point3::new(13.0, 2.0, 3.0))
        .with_look_at(Point3::new(0.0, 0.0, 0.0))
        .with_up(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.6)
        .with_focal_distance(10.0)
        .build();

    camera.render(&world)?;

    Ok(())
}
