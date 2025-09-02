//! Hittable list

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }
    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.objects.push(Box::new(object))
    }
}
impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut current_best = None;
        for object in &self.objects {
            match object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                Some(h) => {
                    closest_so_far = h.t;
                    current_best = Some(h)
                },
                None => ()
            }
        }
        current_best
    }
}