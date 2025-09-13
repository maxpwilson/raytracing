//! Defines textures

use crate::vec3::Point3;
use crate::color::Color;

pub mod checkered;
pub mod image;

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: Point3) -> &Color;
}

pub struct SolidColor {
    albedo: Color,
}
impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: Point3) -> &Color {
        &self.albedo
    }
}
