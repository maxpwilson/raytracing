//! Define 3D Vector

use std::ops;
use crate::random_float;

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn random(min: f64, max: f64) -> Self {
        Vec3::new(random_float(min, max), random_float(min, max), random_float(min, max))
    }


    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(&self) -> Self {
        *self / self.length()    
    }
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        };
    }
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let ruv = Self::random_unit_vector();
        if dot(&ruv, normal) > 0.0 {
            return ruv;
        } else {
            return -ruv;
        }
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub type Point3=Vec3;

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