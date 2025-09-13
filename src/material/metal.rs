//! Defines metal material
use crate::material::{ Material, HitRecord, Ray, Color };
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = r_in.direction.reflect(&rec.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        let scattered_ray = Ray::new(rec.p, reflected, r_in.time);
        let attenuation = self.albedo;
        Some((scattered_ray, attenuation))
    }
}
