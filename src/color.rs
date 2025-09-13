//! Color functions

use anyhow::Result;
use std::io::Write;
use std::ops;
use crate::vec3::Vec3;
use crate::interval::Interval;

/// Defines r,g,b color object
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r: r, g: g, b: b }
    }
}

/// Translate linear color value to gamma corrected value
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

/// Write color value to specified output stream
pub fn write_color(mut out: impl Write, pixel_color: Color) -> Result<()> {
    let r = linear_to_gamma(pixel_color.r);
    let g = linear_to_gamma(pixel_color.g);
    let b = linear_to_gamma(pixel_color.b);

    // Translate [0, 1] components to [0, 255]
    let intensity = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * intensity.itv_clamp(r)) as usize;
    let gbyte = (256.0 * intensity.itv_clamp(g)) as usize;
    let bbyte = (256.0 * intensity.itv_clamp(b)) as usize;
    writeln!(out, "{rbyte} {gbyte} {bbyte}")?;
    Ok(())
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color::new(rhs.r * self.r, rhs.g * self.g, rhs.b * self.b)
    }
}
impl From<Vec3> for Color {
    fn from(v: Vec3) -> Color {
        Color::new(v.x, v.y, v.z)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color::new(rhs.r * self, rhs.g * self, rhs.b * self)
    }
}

#[test]
fn test_add_assign() {
    let mut c = Color::new(0.0, 0.0, 0.0);
    let c1 = Color::new(1.0, 0.0, 0.0);
    let c2 = Color::new(0.0, 2.0, 0.0);
    let c3 = Color::new(0.0, 0.0, 3.0);
    let result = Color::new(1.0, 2.0, 3.0);
    c += c1;
    c += c2;
    c += c3;
    assert_eq!(c, result);
}
