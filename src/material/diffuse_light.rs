//! Material that generates diffuse light

use crate::material::Material;
use crate::color::Color;
use crate::vec3::Point3;
use crate::texture::Texture;

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight<T: Texture> {
    texture: T
} 

impl<T: Texture> DiffuseLight<T> {
    pub fn new(texture: T) -> Self {
        DiffuseLight { texture }
    }
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn emit(&self, u: f64, v: f64, p: Point3) -> Color {
        self.texture.color(u, v, p)
    }
}