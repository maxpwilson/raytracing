//! Texture containing an image

use crate::texture::Texture;
use crate::image::Image;
use crate::color::Color;
use crate::vec3::Point3;
use crate::interval::Interval;

#[derive(Clone)]
pub struct ImageTexture {
    image: Image,
}
impl ImageTexture {
    pub fn new(image: Image) -> Self {
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64, _p: Point3) -> Color {
        if self.image.height <= 0 { return Color::new(0.0, 1.0, 1.0) }
        
        let u = Interval::new(0.0, 1.0).itv_clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).itv_clamp(v);

        let i = (u * self.image.width as f64) as usize;
        let j = (v * self.image.height as f64) as usize;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(color_scale * pixel.0 as f64, color_scale * pixel.1 as f64, color_scale * pixel.2 as f64)
    }
}
