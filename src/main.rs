mod camera;
mod hittable;
mod material;
mod ray;
mod vector_utils;

use crate::camera::Camera;
use crate::hittable::Sphere;
use crate::material::Material;
use anyhow::Result;
use glam::{EulerRot, Vec3A};
use image::ImageFormat;
use std::f32::consts::{FRAC_PI_4, PI};
use std::time::Instant;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

fn main() -> Result<()> {
    let vfov = 90.0 * PI / 180.0;
    let center = Vec3A::new(-2.0, 2.0, 1.0);
    let rotation = glam::Quat::from_euler(EulerRot::YXZ, -FRAC_PI_4, -FRAC_PI_4 + 0.15, 0.0); //yaw, pitch, roll
    let focal_length = 1.7f32;

    let camera = Camera::new(WIDTH, HEIGHT, center, rotation, vfov, focal_length);

    let mut world = vec![];
    let ground_material = Material::Lambertian {
        albedo: Vec3A::new(0.5, 0.5, 0.5),
    };
    let ground = Sphere {
        center: Vec3A::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    };
    world.push(ground);

    let material1 = Material::Metal {
        albedo: Vec3A::new(4.0, 3.0, 1.0),
        fuzz: 0.0
    };
    let sphere1 = Sphere {
        center: Vec3A::new(0.0, 0.5, 0.0),
        radius: 0.5,
        material: material1,
    };
    world.push(sphere1);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3A::new(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere = Sphere {
                    center,
                    radius: 0.2,
                    material: if choose_mat < 0.8 {
                        Material::Lambertian {
                            albedo: Vec3A::new(
                                rand::random::<f32>() * rand::random::<f32>(),
                                rand::random::<f32>() * rand::random::<f32>(),
                                rand::random::<f32>() * rand::random::<f32>(),
                            ),
                        }
                    } else if choose_mat < 0.95 {
                        Material::Metal {
                            albedo: Vec3A::new(
                                0.5 * (1.0 + rand::random::<f32>()),
                                0.5 * (1.0 + rand::random::<f32>()),
                                0.5 * (1.0 + rand::random::<f32>()),
                            ),
                            fuzz: 0.5 * rand::random::<f32>(),
                        }
                    } else {
                        Material::Dielectric {
                            albedo: Vec3A::new(1.0, 1.0, 1.0),
                            refraction_index: 1.5,
                        }
                    },
                };
                world.push(sphere);
            }
        }
    }




    let start = Instant::now();
    let image = camera.render(&world);
    let duration = Instant::now() - start;

    println!("done in {}s, saving file", duration.as_secs_f32());

    image.save_with_format("output3.png", ImageFormat::Png)?;

    Ok(())
}
