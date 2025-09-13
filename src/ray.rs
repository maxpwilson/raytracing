//! Ray functions

use crate::vec3::{ Point3, Vec3 };

#[derive(Debug, Clone)]
/// Ray from origin in given direction
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray { origin: origin, direction: direction, time }
    }
    /// Point along ray at time t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
