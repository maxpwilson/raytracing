//! Defines checkered texture

use crate::texture::{ Texture, SolidColor };
use crate::color::Color;
use crate::vec3::Point3;
use std::rc::Rc;

#[derive(Clone)]
pub struct Checkered {
    inv_scale: f64,
    even_texture: Rc<dyn Texture>,
    odd_texture: Rc<dyn Texture>,
}
impl Checkered {
    pub fn new(
        inv_scale: f64,
        even_texture: Rc<dyn Texture>,
        odd_texture: Rc<dyn Texture>
    ) -> Self {
        Checkered { inv_scale, even_texture, odd_texture }
    }
    pub fn from_solids(inv_scale: f64, even_color: Color, odd_color: Color) -> Self {
        Checkered::new(
            inv_scale,
            Rc::new(SolidColor::new(even_color)),
            Rc::new(SolidColor::new(odd_color))
        )
    }
}

impl Texture for Checkered {
    fn color(&self, u: f64, v: f64, p: Point3) -> &Color {
        let x = (self.inv_scale * p.x).floor() as i32;
        let y = (self.inv_scale * p.y).floor() as i32;
        let z = (self.inv_scale * p.z).floor() as i32;
        if (x + y + z) % 2 == 0 {
            return self.even_texture.color(u, v, p);
        } else {
            return self.odd_texture.color(u, v, p);
        }
    }
}
