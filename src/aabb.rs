//! Implements Axis-aligned bounding boxes

use crate::interval::Interval;
use crate::vec3::Point3;
use crate::ray::Ray;

#[derive(Debug, Clone)]
/// Axis-aligned bounding box
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        // Add minimum padding
        let delta = 0.0001; // minimum size
        let x = match x.size() > delta {
            true => x,
            false => x.expand(delta)
        };
        let y = match y.size() > delta {
            true => y,
            false => y.expand(delta)
        };
        let z = match z.size() > delta {
            true => z,
            false => z.expand(delta)
        };
        Aabb { x, y, z }
    }
    /// Empty box
    pub fn new_zero() -> Self {
        let zero = Interval::new(0.0, 0.0);
        Aabb::new(zero, zero, zero)
    }
    /// Construct bounding box using two points as extrema
    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = Interval::new_bound_check(a.x, b.x);
        let y = Interval::new_bound_check(a.y, b.y);
        let z = Interval::new_bound_check(a.z, b.z);
        Aabb::new(x, y, z)
    }
    /// Construct bounding box using two boxes as extrema
    pub fn from_boxes(box1: &Aabb, box2: &Aabb) -> Aabb {
        let x = Interval::new_enclosing(&box1.x, &box2.x);
        let y = Interval::new_enclosing(&box1.y, &box2.y);
        let z = Interval::new_enclosing(&box1.z, &box2.z);
        Aabb::new(x, y, z)
    }
    pub fn axis(&self, n: i32) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
    /// return interval of longest axis
    pub fn longest_axis(&self) -> i32 {
        if self.x > self.y && self.x > self.z {
            return 0;
        } else if self.y > self.z {
            return 1;
        } else {
            return 2;
        }
    }
    /// Check whether bounding box is hit by ray r
    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<Interval> {
        let ray_orig = r.origin;
        let ray_dir = r.direction;
        let mut ray_t_max = ray_t.max;
        let mut ray_t_min = ray_t.min;

        for axis in 0..3 {
            let ax = self.axis(axis);
            let adinv = 1.0 / ray_dir.axis(axis);

            let t0 = (ax.min - ray_orig.axis(axis)) * adinv;
            let t1 = (ax.max - ray_orig.axis(axis)) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t_min = t0;
                }
                if t1 < ray_t.max {
                    ray_t_max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t_min = t1;
                }
                if t0 < ray_t.max {
                    ray_t_max = t0;
                }
            }
            if ray_t_max <= ray_t_min {
                return None;
            }
        }
        Some(Interval::new(ray_t_min, ray_t_max))
    }
}
