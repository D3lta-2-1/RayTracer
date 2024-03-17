mod camera;
mod hittable;
mod material;
mod ray;
mod vector_utils;

use crate::camera::Camera;
use crate::hittable::Sphere;
use crate::material::{Color, Material};
use anyhow::Result;
use glam::Vec3A;
use image::ImageFormat;
use std::time::Instant;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

fn main() -> Result<()> {
    let camera = Camera::new(WIDTH, HEIGHT);

    let mut world = vec![];
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.2, 0.2, 0.2),
    };

    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Material::Dielectric {
        albedo: Color::new(1.0, 1.0, 1.0),
        refraction_index: 1.5,
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    //for output2.png
    /*let material_center = Material::Dielectric {
        albedo: Color::new(1.0, 0.0, 0.0),
        refraction_index: 1.5,
    };
    let material_left = Material::Dielectric {
        albedo: Color::new(0.0, 1.0, 0.0),
        refraction_index: 1.5,
    };
    let material_right = Material::Dielectric {
        albedo: Color::new(0.0, 0.0, 1.0),
        refraction_index: 1.5,
    };*/

    world.push(Sphere::new(
        Vec3A::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.push(Sphere::new(
        Vec3A::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.push(Sphere::new(Vec3A::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.push(Sphere::new(Vec3A::new(1.0, 0.0, -1.0), 0.5, material_right));

    let start = Instant::now();
    let image = camera.render(&world);
    let duration = Instant::now() - start;

    println!("done in {}s, saving file", duration.as_secs_f32());

    image.save_with_format("output.png", ImageFormat::Png)?;

    Ok(())
}
