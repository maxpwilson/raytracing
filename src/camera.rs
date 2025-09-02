//! Camera code

use crate::ray::Ray;
use crate::color::{Color, write_color};
use crate::interval::Interval;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};
use crate::random_float;

pub struct Camera {
    pub aspect_ratio: f64,         // Ratio of image widht over height
    pub image_width: i32,          // Rendered image width in pixel count
    pub samples_per_pixel: i32,    // Count of random samples for each pixel
    pub max_depth: i32,            // Maximum number of ray bounces
    image_height: i32,             // Rendered image height
    pixel_samples_scale: f64,      // Color scale factor for a sum of pixel samples
    center: Point3,                // Camera center
    pixel00_loc: Point3,           // Location of pixel at 0, 0
    pixel_delta_u: Vec3,           // Offset to pixel to the right
    pixel_delta_v: Vec3            // Offset to pixel below

}

impl Camera {
    fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, 
                    max_depth: i32, image_height: i32, 
                    pixel_samples_scale: f64, center: Point3, pixel00_loc: Point3,
                    pixel_delta_u: Vec3, pixel_delta_v: Vec3) -> Self {
        Camera { aspect_ratio, image_width, samples_per_pixel, max_depth,
            image_height, pixel_samples_scale, center, pixel00_loc, pixel_delta_u, pixel_delta_v }
    }
    pub fn initialize(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        // calculate image height from width and aspect ratio
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1 { image_height = 1; }
        let center = Point3::zero();
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate vectors across the horizontal and down vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        
        // Calculate horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) 
                                        - (viewport_u/2.0) - (viewport_v/2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth,
            image_height, pixel_samples_scale,
            center, pixel00_loc,
            pixel_delta_u, pixel_delta_v)
    }


    pub fn render(&self, world: &impl Hittable) {

        let mut out = std::io::stdout();

        // Render image
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world);
                }
                write_color(&mut out, self.pixel_samples_scale * pixel_color).unwrap();
            }
        }
        eprint!("\rDone                                                   \n");
    }
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc 
                                + ((i as f64 + offset.x) * self.pixel_delta_u) 
                                + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

}

fn sample_square() -> Vec3 {
    Vec3::new(random_float(-0.5, 0.5), random_float(-0.5, 0.5), 0.0)
}

fn ray_color(r: &Ray, depth: i32, world: &impl Hittable) -> Color {
    if depth <= 0{
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
        let direction = Vec3::random_unit_vector() + rec.normal;
        return 0.9 * ray_color(&Ray::new(rec.p, direction), depth-1, world);
    }
    let unit_direction = r.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
}