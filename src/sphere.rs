use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{Point, dot},
};

pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub material: Arc<Box<dyn Material + Sync + Send>>,
}

impl Sphere {
    pub fn new(centre: Point, radius: f64, material: Arc<Box<dyn Material + Sync + Send>>) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.centre - ray.origin;
        let a = ray.direction.magnitude2();
        let h = dot(ray.direction, oc);
        let c = oc.magnitude2() - self.radius * self.radius;

        let discrimant = h * h - a * c;

        if discrimant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discrimant.sqrt();

        let mut root = (h - sqrt_discriminant) / a;
        if !ray_t.surounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surounds(root) {
                return None;
            }
        }
        let point = ray.at(root);
        let mut normal = (point - self.centre) / self.radius;
        let front_face = dot(ray.direction, normal) < 0.0;
        normal = if front_face { normal } else { -normal };

        Some(HitRecord {
            point,
            normal,
            t: root,
            front_face,
            material: self.material.clone(),
        })
    }
}
