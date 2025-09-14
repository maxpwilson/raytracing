//! Library file
use rand::Rng;

pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod interval;
pub mod camera;
pub mod material;
pub mod aabb;
pub mod texture;
pub mod image;
pub mod perlin;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * std::f64::consts::PI) / 180.0
}

pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    let r = rng.random::<f64>();
    min + (max - min) * r
}

pub fn random_int(min: i32, max: i32) -> i32 {
    random_float(min as f64, (max as f64) + 1.0) as i32
}
