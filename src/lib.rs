//! Library file 
use rand::Rng;

pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod interval;
pub mod camera;

const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    let r = rng.random::<f64>();
    min + (max - min)*r
}