//! Define materials

pub mod lambertian;
pub mod metal;
pub mod dialectric;
pub mod diffuse_light;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vec3::Point3;

pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
    fn emit(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
