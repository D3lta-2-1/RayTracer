use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector_utils::{near_zero, random_unit_vector, reflect, reflectance, refract};
use glam::Vec3A;
use rand::{Rng, thread_rng};

pub type Color = Vec3A;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { albedo: Color, refraction_index: f32 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut direction = hit_record.normal + random_unit_vector();
                if near_zero(direction) {
                    direction = hit_record.normal;
                }
                Some((
                    albedo.clone(),
                    Ray {
                        origin: hit_record.point,
                        direction,
                    },
                ))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(r_in.direction, hit_record.normal);
                let direction = reflected + *fuzz * random_unit_vector();
                Some((
                    albedo.clone(),
                    Ray {
                        origin: hit_record.point,
                        direction,
                    },
                ))
            }
            Material::Dielectric { albedo, refraction_index } => {
                //Snell-Descartes' law
                let refraction_ratio = if hit_record.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.direction.normalize();

                let cos_theta = f32::min(Vec3A::dot(-unit_direction, hit_record.normal), 1.0);
                let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let mut rng = thread_rng();
                let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0) {
                    reflect(unit_direction, hit_record.normal)
                } else {
                    refract(unit_direction, hit_record.normal, refraction_ratio)
                };

                Some((
                    *albedo,
                    Ray {
                        origin: hit_record.point,
                        direction,
                    },
                ))
            },
        }
    }
}
