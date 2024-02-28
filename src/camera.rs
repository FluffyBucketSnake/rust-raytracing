use std::{error::Error, io::Write};

use crate::{hittable::Hittable, interval::Interval, prelude::*, Color};

pub type RenderResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct CameraBuilder {
    image_width: usize,
    aspect_ratio: float,
    samples_per_pixel: usize,
    max_depth: usize,
    look_from: Point3,
    look_at: Point3,
    up: Vec3,
    vfov: float,
    focal_distance: float,
    defocus_angle: float,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            look_from: Point3::NEG_Z,
            look_at: Point3::ZERO,
            up: Vec3::Y,
            vfov: 90.0,
            defocus_angle: 0.0,
            focal_distance: 10.0,
            max_depth: 10,
            samples_per_pixel: 10,
            image_width: 100,
            aspect_ratio: 1.0,
        }
    }
}

macro_rules! builder_fn {
    ($field:ident : $type:ty, $func:ident) => {
        #[inline]
        pub fn $func(mut self, value: $type) -> Self {
            self.$field = value;
            self
        }
    };
}

impl CameraBuilder {
    builder_fn!(look_from: Point3, with_look_from);
    builder_fn!(look_at: Point3, with_look_at);
    builder_fn!(up: Vec3, with_up);
    builder_fn!(vfov: float, with_vfov);
    builder_fn!(focal_distance: float, with_focal_distance);
    builder_fn!(defocus_angle: float, with_defocus_angle);
    builder_fn!(max_depth: usize, with_max_depth);
    builder_fn!(samples_per_pixel: usize, with_samples_per_pixel);
    builder_fn!(image_width: usize, with_image_width);
    builder_fn!(aspect_ratio: float, with_aspect_ratio);

    #[inline]
    pub fn build(self) -> Camera {
        let image_height = (((self.image_width as float) / self.aspect_ratio) as usize).max(1);

        let center = self.look_from;

        let real_aspect_ratio = (self.image_width as float) / (image_height as float);
        let h = (self.vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h * self.focal_distance;
        let viewport_width = viewport_height * real_aspect_ratio;

        let w = (self.look_from - self.look_at).unit();
        let u = self.up.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_du = viewport_u / self.image_width as float;
        let pixel_dv = viewport_v / image_height as float;
        let viewport_top_left =
            center - (self.focal_distance * w) - (viewport_u + viewport_v) / 2.0;
        let top_left_pixel = viewport_top_left + 0.5 * (pixel_du + pixel_dv);

        let defocus = (self.defocus_angle > 0.0).then(|| {
            let defocus_radius =
                self.focal_distance * (self.defocus_angle / 2.0).to_radians().tan();
            [defocus_radius * u, defocus_radius * v]
        });

        Camera {
            image_width: self.image_width,
            samples_pex_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            image_height,
            center,
            top_left_pixel,
            pixel_du,
            pixel_dv,
            defocus,
        }
    }
}

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
    defocus: Option<[Vec3; 2]>,
}

impl Camera {
    pub fn render(&self, world: &impl Hittable) -> RenderResult {
        let log = &mut std::io::stderr();
        let output = &mut std::io::stdout();

        write_ppm_header(output, self.image_width, self.image_height)?;
        for j in 0..self.image_height {
            let _ = writeln!(log, "Scanline progress: {}/{}", j, self.image_height);
            for i in 0..self.image_width {
                let color = (0..self.samples_pex_pixel)
                    .map(|_| ray_color(&self.get_ray(i, j), world, self.max_depth))
                    .sum::<Color>()
                    / (self.samples_pex_pixel as float);
                write_ppm_pixel(output, color.gamma_corrected())?;
            }
        }
        let _ = writeln!(log, "Done");
        output.flush()?;
        Ok(())
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.top_left_pixel + (i as float) * self.pixel_du + (j as float) * self.pixel_dv;
        let pixel_sample = pixel_center + self.get_pixel_sample();
        let ray_origin = self.center + self.get_defocus();
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction);
    }

    fn get_pixel_sample(&self) -> Vec3 {
        let px = -0.5 + rand_norm();
        let py = -0.5 + rand_norm();
        (px * self.pixel_du) + (py * self.pixel_dv)
    }

    fn get_defocus(&self) -> Vec3 {
        self.defocus
            .map(|[u, v]| {
                let p = Vec3::random_in_unit_disk();
                return (p.x() * u) + (p.y() * v);
            })
            .unwrap_or_default()
    }
}

fn ray_color(ray: &Ray, world: &impl Hittable, max_depth: usize) -> Color {
    if max_depth == 0 {
        return Color::ZERO;
    }

    if let Some(hit) = world.hit(ray, Interval::new(0.001, float::INFINITY)) {
        return hit
            .material
            .scatter(ray, &hit)
            .map(|(attenuation, scatter)| attenuation * ray_color(&scatter, world, max_depth - 1))
            .unwrap_or(Color::ZERO);
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
