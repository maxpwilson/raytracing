//! Camera code

use crate::ray::Ray;
use crate::color::{ Color, write_color };
use crate::interval::Interval;
use crate::hittable::Hittable;
use crate::vec3::{ Point3, Vec3 };
use crate::random_float;
use crate::degrees_to_radians;

pub struct CameraArgs {
    pub aspect_ratio: f64, // Ratio of image widht over height
    pub image_width: i32, // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32, // Maximum number of ray bounces
    pub vfov: f64, // Vertical view angle
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3, // Point camera is looking at
    pub vup: Vec3, // Camera-relative up direction
    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64, // Distance from camera lookfrom point to plane of perfect focus
}
impl CameraArgs {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64
    ) -> Self {
        CameraArgs {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
        }
    }

    /// Create camera object for rendering based on minimum values required
    pub fn initialize(self) -> Camera {
        // calculate image height from width and aspect ratio
        let mut image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        let pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        let center = self.lookfrom;

        // Determine viewport dimensions
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * ((self.image_width as f64) / (image_height as f64));

        // Calculate u,v,w unit basis vectors for camera coordinated frame
        let w = (self.lookfrom - self.lookat).unit_vector();
        let u = self.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate vectors across the horizontal and down vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / (self.image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - self.focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Camera::new(
            self,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v
        )
    }
}

/// Defines camera used to render
pub struct Camera {
    args: CameraArgs,
    image_height: i32, // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3, // Camera center
    pixel00_loc: Point3, // Location of pixel at 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    fn new(
        args: CameraArgs,
        image_height: i32,
        pixel_samples_scale: f64,
        center: Point3,
        pixel00_loc: Point3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        defocus_disk_u: Vec3,
        defocus_disk_v: Vec3
    ) -> Self {
        Camera {
            args,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    /// Render hittable object on camera
    pub fn render(&self, world: impl Hittable) {
        let mut out = std::io::stdout();

        // Render image
        println!("P3\n{} {}\n255", self.args.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {} ", self.image_height - j);
            for i in 0..self.args.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.args.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.args.max_depth, &world);
                }
                write_color(&mut out, self.pixel_samples_scale * pixel_color).unwrap();
            }
        }
        eprint!("\rDone                                                   \n");
    }
    /// Generate ray with direction toward random point on unit square centered on given pixel
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample =
            self.pixel00_loc +
            ((i as f64) + offset.x) * self.pixel_delta_u +
            ((j as f64) + offset.y) * self.pixel_delta_v;
        let ray_origin = match self.args.defocus_angle <= 0.0 {
            true => self.center,
            false => self.defocus_disk_sample(),
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_float(0.0, 1.0);
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}

/// Create vector on the unit square centered on 0
fn sample_square() -> Vec3 {
    Vec3::new(random_float(-0.5, 0.5), random_float(-0.5, 0.5), 0.0)
}

fn ray_color(r: &Ray, depth: i32, world: &impl Hittable) -> Color {
    // exits after max depth exceeded
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
        match rec.material.scatter(r, &rec) {
            Some((scattered, attenuation)) => {
                return attenuation * ray_color(&scattered, depth - 1, world);
            }
            None => {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
    }

    // defines background if nothing hit
    let unit_direction = r.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
