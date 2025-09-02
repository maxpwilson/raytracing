//! Sphere object implementation

use crate::vec3::{Point3, dot};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Sphere {
    center: Point3,
    radius: f64
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = dot(&r.direction, &oc);
        let c = dot(&oc,& oc) - self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        } 
        let sqrtd = discriminant.sqrt();

        let mut root = (h-sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) { 
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::from_ray(r, p, normal, t))
    }
}   