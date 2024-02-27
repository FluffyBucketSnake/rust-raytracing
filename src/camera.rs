use std::{error::Error, io::Write};

use crate::{hittable::Hittable, interval::Interval, prelude::*, Color};

pub type RenderResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    samples_pex_pixel: usize,
    max_depth: usize,
    center: Point3,
    top_left_pixel: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
}

impl Camera {
    pub fn from_aspect_ratio(
        image_width: usize,
        aspect_ratio: float,
        samples_pex_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let center = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;

        let image_height = (((image_width as float) / aspect_ratio) as usize).max(1);

        let real_aspect_ratio = image_width as float / image_height as float;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * real_aspect_ratio;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_du = viewport_u / image_width as float;
        let pixel_dv = viewport_v / image_height as float;
        let viewport_top_left =
            center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;
        let top_left_pixel = viewport_top_left + 0.5 * (pixel_du + pixel_dv);

        Self {
            image_width,
            image_height,
            samples_pex_pixel,
            max_depth,
            center,
            top_left_pixel,
            pixel_du,
            pixel_dv,
        }
    }

    pub fn render(&self, world: &impl Hittable) -> RenderResult {
        let log = &mut std::io::stderr();
        let output = &mut std::io::stdout();

        write_ppm_header(output, self.image_width, self.image_height)?;
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let color = (0..self.samples_pex_pixel)
                    .map(|_| ray_color(&self.get_ray(i, j), world, self.max_depth))
                    .sum::<Color>()
                    / (self.samples_pex_pixel as float);
                write_ppm_pixel(output, color)?;
            }
            let _ = writeln!(log, "Scanline progress: {}/{}", j, self.image_height);
        }
        let _ = writeln!(log, "Done");
        output.flush()?;
        Ok(())
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.top_left_pixel + (i as float) * self.pixel_du + (j as float) * self.pixel_dv;
        let pixel_sample = pixel_center + self.get_pixel_sample_square();
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn get_pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand_norm();
        let py = -0.5 + rand_norm();
        (px * self.pixel_du) + (py * self.pixel_dv)
    }
}

fn ray_color(ray: &Ray, world: &impl Hittable, max_depth: usize) -> Color {
    if max_depth == 0 {
        return Color::ZERO;
    }

    if let Some(hit) = world.hit(&ray, Interval::new(0.001, float::INFINITY)) {
        let new_direction = Vec3::random_on_hemisphere(&hit.normal);
        return 0.5 * ray_color(&Ray::new(hit.position, new_direction), world, max_depth - 1);
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
