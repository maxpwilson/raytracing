//! Hittable list

use crate::hittable::{ Hittable, HitRecord };
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::Aabb;

use std::rc::Rc;

/// Implement a list of hittable items.
///
/// Each object has the hittable trait and is stored in a Box. Each item
/// must live at least as long as the hittable list -> 'a
pub struct HittableList<'a> {
    pub objects: Vec<Rc<dyn Hittable + 'a>>,
    bounding_box: Aabb,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList { objects: Vec::new(), bounding_box: Aabb::new_zero() }
    }
    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.bounding_box = Aabb::from_boxes(&self.bounding_box, object.bounding_box());
        self.objects.push(Rc::new(object));
    }
}
impl Hittable for HittableList<'_> {
    /// Check if any object in the list was hit by ray r in interval ray_t
    ///
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // self lives for 'a
        let mut closest_so_far = ray_t.max;
        let mut current_best = None;
        for object in &self.objects {
            // object lives for 'b
            match object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                Some(h) => {
                    closest_so_far = h.t;
                    current_best = Some(h);
                }
                None => (),
            }
        }
        current_best
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
