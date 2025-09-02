//! Render an image using ray tracing

use raytracing::vec3::{Point3};
use raytracing::hittable_list::HittableList;
use raytracing::sphere::Sphere;
use raytracing::camera::Camera;


fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 10;
    let max_depth = 10;

    let camera = Camera::initialize(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // World 

    let mut world = HittableList::new();
    let s1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(s1);
    world.add(s2);

    camera.render(&world);

}
