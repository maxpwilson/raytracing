//! 3D box implementation

use crate::hittable::{ Hittable, HitRecord };
use crate::hittable::hittable_list::HittableList;
use crate::hittable::quad::Quad;
use crate::material::Material;
use crate::vec3::{ Point3, Vec3 };
use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Cube<'a> {
    sides: HittableList<'a>,
}
impl<'a> Cube<'a> {
    fn new(sides: HittableList<'a>) -> Self {
        Cube { sides }
    }
    pub fn from_points(a: Point3, b: Point3, material: impl Material + 'a + Copy) -> Self {
        let mut sides = HittableList::new();
        let min_p = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max_p = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3::new(max_p.x - min_p.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max_p.y - min_p.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max_p.z - min_p.z);

        sides.add(Quad::new_static(Point3::new(min_p.x, min_p.y, max_p.z), dx, dy, material));
        sides.add(Quad::new_static(Point3::new(max_p.x, min_p.y, max_p.z), -dz, dy, material));
        sides.add(Quad::new_static(Point3::new(max_p.x, min_p.y, min_p.z), -dx, dy, material));
        sides.add(Quad::new_static(Point3::new(min_p.x, min_p.y, min_p.z), dz, dy, material));
        sides.add(Quad::new_static(Point3::new(min_p.x, max_p.y, max_p.z), dx, -dz, material));
        sides.add(Quad::new_static(Point3::new(min_p.x, min_p.y, min_p.z), dx, dz, material));

        Cube::new(sides)
    }
}

impl<'a> Hittable for Cube<'a> {
    fn bounding_box(&self) -> &Aabb {
        self.sides.bounding_box()
    }
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        self.sides.hit(r, ray_t)
    }
}