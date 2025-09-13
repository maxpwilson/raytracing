//! Defines dialectric material
use crate::material::{ Material, HitRecord, Ray, Color };
use crate::random_float;

#[derive(Clone, Copy)]
pub struct Dialectric {
    refraction_index: f64,
}
impl Dialectric {
    pub fn new(refraction_index: f64) -> Self {
        Dialectric { refraction_index }
    }
}
impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let mut ri = self.refraction_index;
        if rec.front_face {
            ri = 1.0 / self.refraction_index;
        }
        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction = match
            ri * sin_theta > 1.0 ||
            reflectance(cos_theta, ri) > random_float(0.0, 1.0)
        {
            true => unit_direction.reflect(&rec.normal),
            false => unit_direction.refract(&rec.normal, ri),
        };
        let scattered_ray = Ray::new(rec.p, direction, r_in.time);
        Some((scattered_ray, attenuation))
    }
}
/// Calculate reflectance
///
/// Uses Schlick's approximation
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
