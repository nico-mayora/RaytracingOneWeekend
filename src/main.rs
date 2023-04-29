use raytracing::camera::*;
use raytracing::colour::*;
use raytracing::hittable::*;
use raytracing::hittable_list::*;
use raytracing::ray::*;
use raytracing::rtweekend::*;
use raytracing::sphere::*;

use std::io;
use std::io::Write;
use rayon::prelude::*;
use chrono::prelude::*;
use image::{ImageBuffer, RgbImage};
use std::sync::Arc;

fn ray_colour(r: &Ray, world: &dyn Hittable) -> Colour {
    if let Some(rec) = world.hit(r, 0., INFTY) {
        return 0.5 * (rec.normal + Colour::new(1., 1., 1.));
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

fn main() -> std::io::Result<()> {
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

    // Render
    let colour_matrix: Vec<Vec<Colour>> = (0..image_height).into_par_iter().map( |j| {
        print!(".");
        io::stdout().flush().unwrap();

        let row: Vec<Vec3> = (0..image_width).into_par_iter().map(|i| {
            let mut pixel_colour = Colour::new(0., 0., 0.);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand_f64()) / ((image_width - 1) as f64);
                let v = (j as f64 + rand_f64()) / ((image_height - 1) as f64);

                let r = cam.ray_at_offset(u, v);
                pixel_colour += ray_colour(&r, &world);
            }
            pixel_colour
        }).collect();
        row
    }).collect();

    println!("{:?}", colour_matrix);

    for (j, row) in colour_matrix.iter().enumerate() {
        for (i, pixel) in row.iter().enumerate() {
            println!("{} {}", i, j);
            write_to_img(&mut img, *pixel, samples_per_pixel, i as i32, image_height - 1 - j as i32);
        }
    }

    let path = format!("out/{}.png", Utc::now().to_string());
    img.save(path).unwrap();

    eprintln!("\nDone!\n");
    Ok(())
}
