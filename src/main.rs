//! Render an image using ray tracing

use raytracing::vec3::{ Point3, Vec3 };
use raytracing::hittable::hittable_list::HittableList;
use raytracing::hittable::sphere::Sphere;
use raytracing::camera::CameraArgs;
use raytracing::material::lambertian::Lambertian;
use raytracing::material::metal::Metal;
use raytracing::color::Color;
use raytracing::material::dialectric::Dialectric;
use raytracing::random_float;
use std::time::Instant;
use raytracing::hittable::bvh::BvhNode;
use raytracing::hittable::quad::Quad;
use raytracing::texture::{
    SolidColor,
    checkered::CheckeredTexture,
    image::ImageTexture,
    noise::NoiseTexture,
};
use raytracing::material::diffuse_light::DiffuseLight;
use anyhow::Result;
use raytracing::hittable::cube::Cube;

use raytracing::image::Image;

fn main() -> Result<()> {
    let start = Instant::now();
    //checkered_spheres();
    cornell_box()?;
    eprintln!("Took {} Seconds", start.elapsed().as_secs());
    Ok(())
}

fn cornell_box() -> Result<()> {
    let mut world = HittableList::new();
    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));

    world.add(
        Quad::new_static(
            Point3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            green
        )
    );

    world.add(
        Quad::new_static(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            red
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            light
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            white.clone()
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(555.0, 555.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            white.clone()
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            white.clone()
        )
    );

    world.add(
        Cube::from_points(
            Point3::new(130.0, 0.0, 65.0),
            Point3::new(295.0, 165.0, 230.0),
            white.clone()
        )
    );
    world.add(
        Cube::from_points(
            Point3::new(265.0, 0.0, 295.0),
            Point3::new(430.0, 330.0, 460.0),
            white.clone()
        )
    );

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = Color::new(0.0, 0.0, 0.0);
    let vfov = 40.0;
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = CameraArgs::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    ).initialize();
    camera.render(world);
    Ok(())
}

fn simple_light() -> Result<()> {
    let mut world = HittableList::new();
    let pertext = NoiseTexture::new(4.0);
    world.add(
        Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(pertext.clone()))
    );
    world.add(
        Sphere::new_static(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(pertext.clone()))
    );
    let difflight = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));
    world.add(
        Quad::new_static(
            Point3::new(3.0, 1.0, -2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            difflight.clone()
        )
    );
    world.add(Sphere::new_static(Point3::new(0.0, 7.0, 0.0), 2.0, difflight));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.0, 0.0, 0.0);
    let vfov = 20.0;
    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = CameraArgs::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    ).initialize();
    camera.render(world);
    Ok(())
}

fn quads() -> Result<()> {
    let mut world = HittableList::new();
    let left_red = Lambertian::new(SolidColor::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Lambertian::new(SolidColor::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Lambertian::new(SolidColor::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Lambertian::new(SolidColor::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Lambertian::new(SolidColor::new(Color::new(0.2, 0.8, 0.8)));

    world.add(
        Quad::new_static(
            Point3::new(-3.0, -2.0, 5.0),
            Vec3::new(0.0, 0.0, -4.0),
            Vec3::new(0.0, 4.0, 0.0),
            left_red
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(-2.0, -2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            back_green
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(3.0, -2.0, 1.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 4.0, 0.0),
            right_blue
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(-2.0, 3.0, 1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
            upper_orange
        )
    );
    world.add(
        Quad::new_static(
            Point3::new(-2.0, -3.0, 5.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -4.0),
            lower_teal
        )
    );

    let aspect_ratio = 1.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.7, 0.8, 1.0);
    let vfov = 80.0;
    let lookfrom = Point3::new(0.0, 0.0, 9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = CameraArgs::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    ).initialize();
    camera.render(world);
    Ok(())
}

fn perlin_spheres() -> Result<()> {
    let pertext = NoiseTexture::new(4.0);
    let mut world = HittableList::new();
    world.add(
        Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(pertext.clone()))
    );
    world.add(Sphere::new_static(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(pertext)));
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.7, 0.8, 1.0);
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = CameraArgs::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    ).initialize();
    camera.render(world);
    Ok(())
}

fn earth_texture() -> Result<()> {
    let earth_image = Image::from_file("earthmap.jpg")?;
    let earth_texture = ImageTexture::new(earth_image);
    let earth_surface = Lambertian::new(earth_texture);
    let globe = Sphere::new_static(Point3::zero(), 2.0, earth_surface);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.7, 0.8, 1.0);
    let vfov = 20.0;
    let lookfrom = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = CameraArgs::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    ).initialize();
    camera.render(globe);
    Ok(())
}

fn checkered_spheres() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.7, 0.8, 1.0);
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = CameraArgs::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    ).initialize();
    // World
    let mut world = HittableList::new();
    let checker = CheckeredTexture::from_solids(
        3.2,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9)
    );
    world.add(
        Sphere::new_static(Point3::new(0.0, 10.0, 0.0), 10.0, Lambertian::new(checker.clone()))
    );
    world.add(Sphere::new_static(Point3::new(0.0, -10.0, 0.0), 10.0, Lambertian::new(checker)));
    let bvh = BvhNode::from_list(&mut world.objects);
    camera.render(bvh);
}

fn bouncing_sphers() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new(0.7, 0.8, 1.0);
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = CameraArgs::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist
    ).initialize();
    // World
    let mut world = HittableList::new();
    let checker = CheckeredTexture::from_solids(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9)
    );
    let ground_material = Lambertian::new(checker);
    world.add(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float(0.0, 1.0);
            let center = Point3::new(
                (a as f64) + 0.9 * random_float(0.0, 1.0),
                0.2,
                (b as f64) + 0.9 * random_float(0.0, 1.0)
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // 80% chance lambertian
                    let albedo =
                        Color::new(
                            random_float(0.0, 1.0),
                            random_float(0.0, 1.0),
                            random_float(0.0, 1.0)
                        ) *
                        Color::new(
                            random_float(0.0, 1.0),
                            random_float(0.0, 1.0),
                            random_float(0.0, 1.0)
                        );
                    let sphere_material = Lambertian::new(SolidColor::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_float(0.0, 0.5), 0.0);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // 15% chance metal
                    let albedo = Color::new(
                        random_float(0.5, 1.0),
                        random_float(0.5, 1.0),
                        random_float(0.5, 1.0)
                    );
                    let fuzz = random_float(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new_static(center, 0.2, sphere_material));
                } else {
                    // 5% chance dialectric
                    let sphere_material = Dialectric::new(1.5);
                    world.add(Sphere::new_static(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dialectric::new(1.5);
    let material2 = Lambertian::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new_static(Point3::new(0.0, 1.0, 0.0), 1.0, material1));
    world.add(Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(Sphere::new_static(Point3::new(4.0, 1.0, 0.0), 1.0, material3));
    let bvh = BvhNode::from_list(&mut world.objects);
    camera.render(bvh);
}
