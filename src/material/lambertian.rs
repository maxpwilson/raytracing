//! Defines lambertian material

use crate::material::{ Material, HitRecord, Ray, Color };
use crate::texture::Texture;
use crate::vec3::Vec3;


#[derive(Clone, Copy)]
pub struct Lambertian<T: Texture> {
    texture: T,
}
impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Lambertian { texture }
    }
}
impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered_ray = Ray::new(rec.p, scatter_direction, r_in.time);
        let attenuation = self.texture.color(rec.u, rec.v, rec.p).to_owned();
        Some((scattered_ray, attenuation))
    }
}
