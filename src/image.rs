//! Module for loading images

use image::ImageReader;
use anyhow::Result;

use crate::interval::Interval;

#[derive(Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<u8>,
}

impl Image {
    fn new(width: usize, height: usize, bytes: Vec<u8>) -> Self {
        Image { width, height, bytes }
    }
    pub fn from_file(filename: &str) -> Result<Self> {
        let img = ImageReader::open(filename)?.decode()?;
        let width = img.width() as usize;
        let height = img.height() as usize;
        let bytes = img.as_bytes().to_vec();
        Ok(Image::new(width, height, bytes))
    }
    pub fn pixel_data(&self, i: usize, j: usize) -> (u8, u8, u8) {
        let x = Interval::new(0.0, (self.width - 1) as f64).itv_clamp(i as f64) as usize;
        let y = Interval::new(0.0, (self.height - 1) as f64).itv_clamp(j as f64) as usize;
        let bytes_per_pixel = 3 as usize;
        let bytes_per_scanline = (self.width * bytes_per_pixel) as usize;
        let idx = (y * bytes_per_scanline) + (x * bytes_per_pixel);
        (self.bytes[idx], self.bytes[idx+1], self.bytes[idx+2])
    }
}
