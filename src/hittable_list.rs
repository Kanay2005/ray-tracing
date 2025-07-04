use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Arc<Box<dyn Hittable>>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::<Arc<Box<dyn Hittable>>>::new(),
        }
    }

    pub fn add(&mut self, object: Arc<Box<dyn Hittable>>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}
