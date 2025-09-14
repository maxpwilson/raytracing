//! Texture containing random noise

use crate::texture::{ Texture };
use crate::color::Color;
use crate::vec3::Point3;
use crate::perlin::PerlinGenerator;

const N: usize = 256;

#[derive(Clone)]
pub struct NoiseTexture {
    noise_generator: PerlinGenerator<N>,
    scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        let  noise_generator = PerlinGenerator::<N>::init();
        NoiseTexture { noise_generator, scale }
    }
}

impl Texture for NoiseTexture {
    fn color(&self, _u: f64, _v: f64, p: Point3) -> Color {
        //0.5 * (1.0 + self.noise_generator.noise(self.scale * p)) * Color::new(1.0, 1.0, 1.0)
        //self.noise_generator.turb(p, 7) * Color::new(1.0, 1.0, 1.0) 
        (1.0 + (self.noise_generator.turb(p, 7)*10.0 + p.z*self.scale).sin()) * Color::new(0.5, 0.5, 0.5)
    }
}
