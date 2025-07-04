use std::sync::Arc;

use crate::{interval::Interval, material::Material, ray::*};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<Box<dyn Material + Sync + Send>>,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
