//! Sphere object implementation

use crate::vec3::{ Point3, Vec3 };
use crate::hittable::{ HitRecord, Hittable };
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::Aabb;

#[derive(Debug, Clone)]
/// Defines sphere of a specific material T
/// Center moves along a Ray. For a static sphere
/// the ray has direction 0, 0, 0
pub struct Sphere<T: Material> {
    center: Ray,
    radius: f64,
    material: T,
    bounding_box: Aabb,
}
impl<T: Material> Sphere<T> {
    pub fn new(center: Ray, radius: f64, material: T, bounding_box: Aabb) -> Self {
        Sphere { center, radius, material, bounding_box }
    }
    /// Creates new fixed radius unmoving sphere.
    pub fn new_static(center: Point3, radius: f64, material: T) -> Self {
        let radius_vec = Vec3::new(radius, radius, radius);
        let bounding_box = Aabb::from_points(center - radius_vec, center + radius_vec);
        Sphere::new(Ray::new(center, Vec3::new(0.0, 0.0, 0.0), 0.0), radius, material, bounding_box)
    }
    /// Creates new fixed radius sphere moving between center1 and center2
    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, material: T) -> Self {
        let r = Ray::new(center1, center2 - center1, 0.0);
        let radius_vec = Vec3::new(radius, radius, radius);
        let bounding_box1 = Aabb::from_points(center1 - radius_vec, center1 + radius_vec);
        let bounding_box2 = Aabb::from_points(center2 - radius_vec, center2 + radius_vec);
        let bounding_box = Aabb::from_boxes(&bounding_box1, &bounding_box2);
        Sphere::new(r, radius, material, bounding_box)
    }
    /// Find u, v coordinates on the unit sphere centered at the origin given a point on it, p
    fn get_uv(p: Point3) -> SphereCoords {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        SphereCoords::new(u, v)
    }
}
struct SphereCoords {
    u: f64,
    v: f64,
}
impl SphereCoords {
    fn new(u: f64, v: f64) -> Self {
        SphereCoords { u, v }
    }
}

impl<T: Material> Hittable for Sphere<T> {
    /// Check if sphere was hit by ray r in interval ray_t
    ///
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let current_center = self.center.at(r.time);
        let oc = current_center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let normal = (p - current_center) / self.radius;
        let sphere_coords = Sphere::<T>::get_uv(normal);
        Some(
            HitRecord::from_ray(
                r,
                p,
                normal,
                t,
                sphere_coords.u,
                sphere_coords.v,
                Box::new(&self.material)
            )
        )
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
