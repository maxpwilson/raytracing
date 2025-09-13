//! Bounding volume heirarchy definition
//!
//! Bounding volume is hittable so included in hittable trait

use crate::hittable::{ Hittable, HitRecord };
use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::random_int;
use std::cmp::Reverse;
use std::rc::Rc;

pub struct BvhNode {
    left_node: Rc<dyn Hittable>,
    right_node: Rc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(
        left_node: Rc<dyn Hittable>,
        right_node: Rc<dyn Hittable>,
        bounding_box: Aabb
    ) -> Self {
        BvhNode { left_node, right_node, bounding_box }
    }
    pub fn from_list(hlist: &mut [Rc<dyn Hittable>]) -> Self {
        let comparator = random_int(0, 2);
        let object_span = hlist.len();
        let left_node;
        let right_node;
        if object_span == 1 {
            left_node = hlist.get(0).unwrap().clone();
            right_node = hlist.get(0).unwrap().clone();
        } else if object_span == 2 {
            left_node = hlist.get(0).unwrap().clone();
            right_node = hlist.get(1).unwrap().clone();
        } else {
            hlist.sort_by_key(|x| Reverse(*x.bounding_box().axis(comparator)));
            let mid = object_span / 2;
            left_node = Rc::new(BvhNode::from_list(&mut hlist[0..mid]));
            right_node = Rc::new(BvhNode::from_list(&mut hlist[mid..object_span]));
        }
        let bounding_box = Aabb::from_boxes(left_node.bounding_box(), right_node.bounding_box());
        BvhNode::new(left_node, right_node, bounding_box)
    }
}

impl Hittable for BvhNode {
    /// Check if a hit occurs.
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // check if bounding box is hit
        match self.bounding_box.hit(r, ray_t) {
            // if box hit check both children for hits
            Some(_hit_itv) => {
                let hit_left = self.left_node.hit(r, ray_t);
                let right_max_itv = match &hit_left {
                    None => ray_t.max,
                    Some(rec) => rec.t,
                };
                let hit_right = self.right_node.hit(r, Interval::new(ray_t.min, right_max_itv));
                hit_left.or(hit_right)
            }
            // return None if box not hit
            None => None,
        }
    }

    /// Construct bounding box.
    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
