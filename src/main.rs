use raytracing::camera::*;
use raytracing::colour::*;
use raytracing::hittable::*;
use raytracing::hittable_list::*;
use raytracing::ray::*;
use raytracing::rtweekend::*;
use raytracing::sphere::*;

use chrono::prelude::*;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;

fn ray_colour(r: &Ray, world: &dyn Hittable) -> Colour {
    if let Some(rec) = world.hit(r, 0., INFTY) {
        return 0.5 * (rec.normal + Colour::new(1., 1., 1.));
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height as u32);

    //World
    let mut world = HittableList::new_empty();
    world.add(Arc::new(Sphere {
        centre: Point3::new(0., 0., -1.),
        radius: 0.5,
    }));
    world.add(Arc::new(Sphere {
        centre: Point3::new(0., -100.5, -1.),
        radius: 100.,
    }));
    let world = world;

    // Camera
    let cam = Camera::new();

    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% {eta} remaining",
        )
        .unwrap(),
    );

    // Render
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

                        let r = cam.ray_at_offset(u, v);
                        pixel_colour += ray_colour(&r, &world);
                    }
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
                image_height - 1 - j as i32,
            );
        }
    }

    let path = format!("out/{}.png", Utc::now().to_string());
    img.save(path).unwrap();

    eprintln!("\nDone!\n");
}
