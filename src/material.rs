//! Define materials

pub mod lambertian;
pub mod metal;
pub mod dialectric;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
