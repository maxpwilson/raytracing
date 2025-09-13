//! Hittable objects

use crate::vec3::{ Vec3, Point3 };
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::Aabb;

// Add mods with types of hittable objects
pub mod hittable_list;
pub mod sphere;
pub mod bvh;

/// Parameters of the hit. Material is borrowed from object hit. The material must live at least as long as the
/// HitRecord does -> 'a
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
    pub material: Box<&'a dyn Material>, // borrowed from object hit
}
impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        u: f64,
        v: f64,
        material: Box<&'a dyn Material>
    ) -> Self {
        HitRecord { p, normal, t, front_face, u, v, material }
    }
    /// Build a hit record from a ray. Automatically detects whether the hit is front-facing and
    /// adjusts the normal accordingly
    pub fn from_ray(
        r: &Ray,
        p: Point3,
        outer_normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        material: Box<&'a dyn Material>
    ) -> Self {
        let front_face = r.direction.dot(&outer_normal) < 0.0;
        let normal = match front_face {
            true => outer_normal,
            false => -outer_normal,
        };
        HitRecord::new(p, normal, t, front_face, u, v, material)
    }
}

/// Trait indicating object that can be hit by a ray.
pub trait Hittable {
    /// Check if a hit occurs.
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>>;

    /// Construct bounding box.
    fn bounding_box(&self) -> &Aabb;
}
