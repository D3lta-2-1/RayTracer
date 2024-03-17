use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;
use std::ops::Range;

pub struct HitRecord<'a> {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub material: &'a Material,
    pub t: f32,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Vec3A,
        outward_normal: Vec3A,
        t: f32,
        ray: &Ray,
        material: &'a Material,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closet_so_far = t_range.end;

        for object in self {
            let optional_hit = object.hit(ray, t_range.start..closet_so_far);
            if let Some(hit) = &optional_hit {
                closet_so_far = hit.t;
                closest_hit = optional_hit;
            }
        }
        closest_hit
    }
}

pub enum HittableElement {
    Sphere(Sphere),
}

impl Hittable for HittableElement {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        match self {
            HittableElement::Sphere(sphere) => sphere.hit(ray, t_range),
        }
    }
}

pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32, material: Material) -> HittableElement {
        HittableElement::Sphere(Self {
            center,
            radius,
            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        };

        let sqrt_discriminant = discriminant.sqrt();

        //find the closest root
        let mut root = (-half_b - sqrt_discriminant) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord::new(point, normal, root, ray, &self.material))
    }
}
