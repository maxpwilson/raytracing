//! 2D quadrilateral object implementation

use crate::vec3::{ Point3, Vec3 };
use crate::hittable::{ HitRecord, Hittable };
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::Aabb;


#[derive(Debug, Clone)]
pub struct Quad<T: Material> {
    q: Point3, //first vertice
    u: Vec3, //vector from q to adjacent vertex
    v: Vec3, //vector from q to adjacent vertex
    w: Vec3, //vector related to planar coordinates of quad
    normal: Vec3,
    d: f64, //defines the plane of the quadrilateral
    material: T,
    bounding_box: Aabb,
}

impl<T: Material> Quad<T> {
    pub fn new(
        q: Point3,
        u: Vec3,
        v: Vec3,
        w: Vec3,
        normal: Vec3,
        d: f64,
        material: T,
        bounding_box: Aabb
    ) -> Self {
        Quad { q, u, v, w, normal, d, material, bounding_box }
    }
    pub fn new_static(q: Point3, u: Vec3, v: Vec3, material: T) -> Self {
        //Compute bounding box
        let aabb_diag1 = Aabb::from_points(q, q + u + v);
        let aabb_diag2 = Aabb::from_points(q + u, q + v);
        let bounding_box = Aabb::from_boxes(&aabb_diag1, &aabb_diag2);
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);
        Quad::new(q, u, v, w, normal, d, material, bounding_box)
    }
}

impl<T: Material> Hittable for Quad<T> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let denom = self.normal.dot(&r.direction);

        // check if ray is parallel
        if denom.abs() < 1e-8 {
            return None;
        }

        // Check if hit is outside of interval
        let t = (self.d - self.normal.dot(&r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // check if ray hits within planar coordinates
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        let planar_coords = get_planar_coords(alpha, beta);
        match planar_coords {
            None => {
                return None;
            }
            Some((u, v)) => {
                return Some(
                    HitRecord::from_ray(
                        r,
                        intersection,
                        self.normal,
                        t,
                        u,
                        v,
                        Box::new(&self.material)
                    )
                );
            }
        }
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

/// Returns hit coordinates u, v if hit occurs within plane, otherwise returns None
fn get_planar_coords(alpha: f64, beta: f64) -> Option<(f64, f64)> {
    let unit_itv = Interval::new(0.0, 1.0);
    if unit_itv.contains(alpha) && unit_itv.contains(beta) {
        return Some((alpha, beta));
    }
    None
}
