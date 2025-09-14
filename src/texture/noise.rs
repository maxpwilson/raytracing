//! Texture containing random noise

use crate::texture::{ Texture };
use crate::color::Color;
use crate::vec3::Point3;
use crate::perlin::PerlinGenerator;

const N: usize = 256;

#[derive(Clone)]
pub struct NoiseTexture {
    noise_generator: PerlinGenerator<N>,
}
impl NoiseTexture {
    pub fn new() -> Self {
        let  noise_generator = PerlinGenerator::<N>::init();
        NoiseTexture { noise_generator }
    }
}

impl Texture for NoiseTexture {
    fn color(&self, _u: f64, _v: f64, p: Point3) -> Color {
       self.noise_generator.noise(p) * Color::new(1.0, 1.0, 1.0)
    }
}
