use raytracing::camera::*;

use raytracing::colour::*;
use raytracing::hittable::*;
use raytracing::hittable_list::*;
use raytracing::material::*;
use raytracing::ray::*;
use raytracing::rtweekend::*;
use raytracing::sphere::*;
use raytracing::vec3rtext::*;
use raytracing::viewport::*;

use chrono::prelude::*;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

fn ray_colour(r: &Ray, world: &dyn Hittable, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, 0.001, INFTY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            return attenuation.mul(&ray_colour(&scattered, world, depth - 1));
        }
        return Colour::new(0., 0., 0.);
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

fn main() {
    // Image

    let aspect_ratio = 16. / 9.;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height as u32);
    let max_depth = 20;

    // World

    let mut world = HittableList::new_empty();

    let material_ground = Arc::new(Lambertian {
        albedo: Colour::new(0.8, 0.8, 0.),
    });
    let material_centre = Arc::new(Lambertian {
        albedo: Colour::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dielectric { ir: 1.5 });
    let material_right = Arc::new(Metal {
        albedo: Colour::new(0.8, 0.6, 0.2),
        fuzz: 0.,
    });

    world.add(Arc::new(Sphere {
        centre: Point3::new(0., -100.5, -1.),
        radius: 100.,
        mat: material_ground,
    }));
    world.add(Arc::new(Sphere {
        centre: Point3::new(0., 0., -1.),
        radius: 0.5,
        mat: material_centre,
    }));
    world.add(Arc::new(Sphere {
        centre: Point3::new(-1., 0., -1.),
        radius: 0.5,
        mat: material_left.clone(),
    }));
    world.add(Arc::new(Sphere {
        centre: Point3::new(-1., 0., -1.),
        radius: -0.4,
        mat: material_left.clone(),
    }));
    world.add(Arc::new(Sphere {
        centre: Point3::new(1., 0., -1.),
        radius: 0.5,
        mat: material_right,
    }));
    let world = world; // unmut

    // Camera
    let lookfrom = Point3::new(3., 3., 2.);
    let lookat = Point3::new(0., 0., -1.);

    let cam = Camera::new(
        lookfrom,
        lookat,
        Point3::new(0., 1., 0.),    // vup
        20.,                        // vfov
        aspect_ratio,
        2.,                         // aperture
        (lookfrom - lookat).norm(), // focus_dist
    );

    // Show the scene as it's rendered in real time
    let mut viewport =
        ViewportRenderer::new(image_width as u32, image_height as u32, samples_per_pixel);
    let (sender, receiver) = mpsc::sync_channel::<ColourPosition>(10000000);
    let viewport_thread_handle = thread::spawn(move || viewport.show_rendered_scene(receiver));

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

                    sender
                        .send(ColourPosition {
                            colour: pixel_colour,
                            point: (i, j as u32),
                        })
                        .unwrap();
                    pixel_colour
                })
                .collect();
            pb.inc(1);
            row
        })
        .collect();

    pb.finish();

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

    drop(sender);
    let path = format!("out/{}.png", Utc::now().to_string());
    img.save(path).unwrap();

    eprintln!("\nDone!\n");
    viewport_thread_handle.join().unwrap();
}
