mod hittable;
mod hittable_list;
mod interval;
mod prelude;
mod ray;
mod sphere;
mod vec3;

use std::error::Error;
use std::io::Write;

use hittable::Hittable;
use hittable_list::HittableList;
use interval::Interval;
use prelude::*;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

type Color = Vec3;
type RenderResult = Result<(), Box<dyn Error>>;

fn main() -> RenderResult {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as float) / aspect_ratio) as usize;

    let real_aspect_ratio = image_width as float / image_height as float;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * real_aspect_ratio;

    let camera_center = Point3::default();
    let focal_length = 1.0;

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let viewport_du = viewport_u / image_width as float;
    let viewport_dv = viewport_v / image_height as float;
    let viewport_top_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;
    let pixel_top_left = viewport_top_left + 0.5 * (viewport_du + viewport_dv);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let log = &mut std::io::stderr();
    let output = &mut std::io::stdout();

    write_ppm_header(output, image_width, image_height)?;
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel_top_left + (i as float) * viewport_du + (j as float) * viewport_dv;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(ray, &world);
            write_ppm_pixel(output, color)?;
        }
        let _ = writeln!(log, "Scanline progress: {}/{}", j + 1, image_height);
    }
    output.flush()?;
    Ok(())
}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    if let Some(hit) = world.hit(&ray, Interval::NOT_NEGATIVE) {
        return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
    }

    let direction = ray.direction().unit();
    let a = 0.5 * (direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn write_ppm_header(output: &mut impl Write, width: usize, height: usize) -> RenderResult {
    writeln!(output, "P3")?;
    writeln!(output, "{} {}", width, height)?;
    writeln!(output, "255")?;
    Ok(())
}

fn write_ppm_pixel(output: &mut impl Write, color: Color) -> RenderResult {
    let r = (color.r() * 256.0) as u8;
    let g = (color.g() * 256.0) as u8;
    let b = (color.b() * 256.0) as u8;
    writeln!(output, "{} {} {}", r, g, b)?;
    Ok(())
}
