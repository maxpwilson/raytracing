//! Hittable objects

use crate::vec3::{Vec3, Point3, dot};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64, 
    pub front_face: bool
}
impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        HitRecord { p, normal, t, front_face }
    }
    pub fn from_ray(r: &Ray, p: Point3, outer_normal: Vec3, t: f64) -> Self {
        let front_face = dot(&r.direction, &outer_normal) < 0.0;
        let normal = match front_face {
            true => outer_normal,
            false => -outer_normal
        };
        HitRecord::new(p, normal, t, front_face)
    }
}


pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}