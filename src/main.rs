mod ray;
mod vec3;

use std::error::Error;
use std::io::Write;

use ray::Ray;
use vec3::{Point3, Vec3};

type Color = Vec3;
type RenderResult = Result<(), Box<dyn Error>>;

fn main() -> RenderResult {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f32) / aspect_ratio) as usize;

    let real_aspect_ratio = image_width as f32 / image_height as f32;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * real_aspect_ratio;

    let camera_center = Point3::default();
    let focal_length = 1.0;

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let viewport_du = viewport_u / image_width as f32;
    let viewport_dv = viewport_v / image_height as f32;
    let viewport_top_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;
    let pixel_top_left = viewport_top_left + 0.5 * (viewport_du + viewport_dv);

    let log = &mut std::io::stderr();
    let output = &mut std::io::stdout();

    write_ppm_header(output, image_width, image_height)?;
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel_top_left + (i as f32) * viewport_du + (j as f32) * viewport_dv;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(pixel_center, ray_direction);

            let color = ray_color(ray);
            write_ppm_pixel(output, color)?;
        }
        let _ = writeln!(log, "Scanline progress: {}/{}", j + 1, image_height);
    }
    output.flush()?;
    Ok(())
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let direction = ray.direction().unit();
    let a = 0.5 * (direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3, radius: f32, ray: Ray) -> bool {
    let center_to_origin = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * ray.direction().dot(&center_to_origin);
    let c = center_to_origin.dot(&center_to_origin) - radius * radius;
    let delta = b * b - 4.0 * a * c;
    return delta >= 0.0;
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
