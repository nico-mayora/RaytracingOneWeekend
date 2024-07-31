use super::camera::*;

use crate::util::{colour::*, rtweekend::*, vec3rtext::*};
use crate::config::Settings;

use super::hittable::*;
use super::hittable_list::*;
use super::material::*;
use super::ray::*;
use super::sphere::*;

use chrono::prelude::*;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::*;
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct ColourPosition {
    pub colour: Colour,
    pub point: (u32, u32),
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new_empty();

    let ground_material = Arc::new(Lambertian {
        albedo: Colour::new(0.5, 0.5, 0.5),
    });

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();

            let centre = Point3::new(
                a as f64 + rand::random::<f64>() * 0.9,
                0.2,
                b as f64 + rand::random::<f64>() * 0.9,
            );

            if (centre - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                let mut displacement = Vec3::zeros();
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.75 {
                    // diffuse
                    displacement = Vec3::new(0., rand::thread_rng().gen_range(0.0..0.5), 0.);
                    let albedo = random_vec3().mul(&random_vec3());
                    Arc::new(Lambertian { albedo })
                } else if choose_mat < 0.92 {
                    // metal
                    let albedo = random_vec3();
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    Arc::new(Metal { albedo, fuzz })
                } else {
                    // glass
                    Arc::new(Dielectric { ir: 1.5 })
                };

                world.add(Arc::new(Sphere {
                    centre0: centre,
                    radius: 0.2,
                    mat: sphere_material,
                    displacement,
                }));
            }
        }
    }

    let mat1 = Arc::new(Dielectric { ir: 1.5 });
    world.add(Arc::new(Sphere {
        centre0: Point3::new(0., 1., 0.),
        displacement: Vec3::zeros(),
        radius: 1.,
        mat: mat1,
    }));

    let mat2 = Arc::new(Lambertian {
        albedo: Colour::new(0.4, 0.2, 0.1),
    });
    world.add(Arc::new(Sphere {
        centre0: Point3::new(-4., 1., 0.),
        displacement: Vec3::zeros(),
        radius: 1.,
        mat: mat2,
    }));

    let mat3 = Arc::new(Metal {
        albedo: Colour::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    });
    world.add(Arc::new(Sphere {
        centre0: Point3::new(4., 1., 0.),
        displacement: Vec3::zeros(),
        radius: 1.,
        mat: mat3,
    }));

    world
}

fn ray_colour(r: &Ray, world: &dyn Hittable, depth: u32) -> Colour {
    if depth == 0 {
        return Colour::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            return attenuation.mul(&ray_colour(&scattered, world, depth - 1));
        }
        return Colour::new(0., 0., 0.);
    }

    // Sky
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

pub fn render() {
    // Image

    let settings: Settings = Default::default();

    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.width;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = settings.samples_per_pixel;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height as u32);
    let max_depth = settings.max_depth;

    // World

    println!("Generating random scene...");
    let world = random_scene();
    println!("Scene generation complete! Rendering...");

    // Camera
    let lookfrom = settings.lookfrom;
    let lookat = settings.lookat;

    let vup = settings.vup;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        settings.vfov,
        aspect_ratio,
        settings.aperture,
        settings.focus_distance,
    );

    // Progress bar initialisation
    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% {eta} remaining",
        )
        .unwrap(),
    );

    // Render (every pixel is rendered in parallel thanks to Rayon)
    let colour_matrix: Vec<Vec<Colour>> = (0..image_height)
        .into_par_iter()
        .map(|j| {
            let row: Vec<Vec3> = (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_colour = Colour::new(0., 0., 0.);

                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + rand::random::<f64>()) / ((image_width - 1) as f64);
                        let v = (j as f64 + rand::random::<f64>()) / ((image_height - 1) as f64);

                        let r = cam.get_ray(u, v);

                        pixel_colour += ray_colour(&r, &world, max_depth);
                    }
                    // It's okay to panic if this fails
                    pixel_colour
                })
                .collect();
            pb.inc(1);
            row
        })
        .collect();

    pb.finish();
    eprintln!("\nDone rendering!\nGenerating image...\n");

    for (j, row) in colour_matrix.iter().enumerate() {
        for (i, pixel) in row.iter().enumerate() {
            write_to_img(
                &mut img,
                *pixel,
                samples_per_pixel,
                i as i32,
                image_height - 1 - j as i32, // png is flipped
            );
        }
    }

    let path = format!("out/{}.png", Utc::now());
    img.save(path).unwrap();

    eprintln!("\nImage saved!\n");
    // viewport_thread_handle.join().unwrap();
}
