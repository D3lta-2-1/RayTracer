use glam::Vec3A;
use rand::{thread_rng, Rng};
use std::ops::{Neg, Range};

pub fn random_vector(range: Range<f32>) -> Vec3A {
    let mut rng = thread_rng();
    Vec3A::new(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range),
    )
}

fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p = random_vector(-1.0..1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3A {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: Vec3A) -> Vec3A {
    let vec = random_unit_vector();
    if Vec3A::dot(normal, random_unit_vector()) > 0.0 {
        vec
    } else {
        -vec
    }
}

pub fn near_zero(vec: Vec3A) -> bool {
    vec.x < f32::EPSILON && vec.y < f32::EPSILON && vec.z < f32::EPSILON
}

pub fn reflect(v: Vec3A, normal: Vec3A) -> Vec3A {
    v - 2.0 * Vec3A::dot(v, normal) * normal
}

pub fn refract(uv: Vec3A, normal: Vec3A, etai_over_etat: f32) -> Vec3A {
    let cos_theta = f32::min(Vec3A::dot(-uv, normal), 1.0);
    let r_out_perpendicular = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = (1.0 - r_out_perpendicular.length_squared())
        .abs()
        .sqrt()
        .neg()
        * normal;
    r_out_perpendicular + r_out_parallel
}

///use Schlick's approximation for reflectance
pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi( 5)
}
