//! Define 3D Vector

use std::ops;
use crate::random_float;

/// Generic 3 dimensional vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn axis(&self, n: i32) -> f64 {
        match n {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }
    /// Generate vector given random components with values from min to max
    pub fn random(min: f64, max: f64) -> Self {
        Vec3::new(random_float(min, max), random_float(min, max), random_float(min, max))
    }

    /// Equivalent to dot product of vector with itself
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// Vector length by sum of squares
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    /// Resize dimensions to have length 1
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
    /// Check if vector is near zero
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    /// Reflect off surface with normal vector n
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * *n
    }
    /// Refraction
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-self.dot(n), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }

    /// Dot product of two vectors
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    /// Cross product of two vectors
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }

    /// Generate random unit vector
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }
    /// Generate random unit vector on same hemisphere as normal vector
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let ruv = Self::random_unit_vector();
        if ruv.dot(normal) > 0.0 {
            return ruv;
        } else {
            return -ruv;
        }
    }
    /// Generate random vector in unit disk
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

/// Wrapper specifying point in space rather than vector
pub type Point3 = Vec3;

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
#[test]
fn test_add() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    let y = Vec3::new(2.0, 1.0, 3.0);
    let result = Vec3::new(3.0, 3.0, 6.0);
    assert_eq!(x + y, result);
}

#[test]
fn test_mul() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    let t = 5.0;
    let result = Vec3::new(5.0, 10.0, 15.0);
    assert_eq!(t * x, result);
}

#[test]
fn test_near_zero() {
    let x = Vec3::new(1e-9, 1e-9, 1e-9);
    let y = Vec3::new(1.0, 1e-10, 2.0);
    assert!(x.near_zero());
    assert_ne!(y.near_zero(), true);
}
