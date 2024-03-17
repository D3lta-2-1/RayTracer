use crate::hittable::Hittable;
use crate::material::Color;
use crate::ray::Ray;
use glam::Vec3A;
use image::RgbaImage;
use rand::{thread_rng, Rng};
use rayon::iter::ParallelIterator;

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    camera_center: Vec3A,
    pixel00_loc: Vec3A,
    pixel_delta_u: Vec3A,
    pixel_delta_v: Vec3A,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let focal_length = 1f32;
        let viewport_height = 2f32; //chosen arbitrary
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let camera_center = Vec3A::splat(0.0);

        let viewport_u = Vec3A::new(viewport_width, 0.0, 0.0); //might move this into the camera
        let viewport_v = Vec3A::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = camera_center
            - Vec3A::new(0.0, 0.0, focal_length) //because Z- is forward
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            sample_per_pixel: 500,
            max_depth: 400,
            camera_center,
            pixel00_loc,
            pixel_delta_v,
            pixel_delta_u,
        }
    }

    pub fn get_ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let pixel_sample_squared = || {
            let mut rng = thread_rng();
            let px = rng.gen_range(-0.5f32..0.5);
            let py = rng.gen_range(-0.5f32..0.5);
            px * self.pixel_delta_u + py * self.pixel_delta_v
        };

        let origin = self.camera_center;
        let pixel_center =
            self.pixel00_loc + (x as f32 * self.pixel_delta_u) + (y as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + pixel_sample_squared();

        let direction = pixel_sample - origin;
        Ray { direction, origin }
    }

    fn ray_color(ray: &Ray, world: &impl Hittable, depth: u32) -> Color {
        if depth <= 0 {
            return Vec3A::splat(0.0);
        }

        let hit = world.hit(ray, 0.00001..f32::MAX);
        if let Some(hit) = hit {
            if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            }
            return Color::splat(0.0);
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::splat(1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &(impl Hittable + Sync)) -> RgbaImage {
        let mut image = RgbaImage::new(self.image_width, self.image_height);

        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let mut color = Vec3A::splat(0.0);

            for _ in 0..self.sample_per_pixel {
                let ray = self.get_ray_for_pixel(x, y);
                color += Self::ray_color(&ray, world, self.max_depth);
            }

            let scale = 1.0 / self.sample_per_pixel as f32;
            let linear_to_gamma = |x: f32| x.sqrt();

            color *= scale;
            color.x = linear_to_gamma(color.x);
            color.y = linear_to_gamma(color.y);
            color.z = linear_to_gamma(color.z);

            let rgb_color = (color * 255.0)
                .ceil()
                .clamp(Vec3A::splat(0.0), Vec3A::splat(255.0));

            pixel.0[0] = rgb_color.x as u8;
            pixel.0[1] = rgb_color.y as u8;
            pixel.0[2] = rgb_color.z as u8;
            pixel.0[3] = 255;
        });
        image
    }
}
