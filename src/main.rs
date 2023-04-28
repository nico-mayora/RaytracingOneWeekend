use raytracing::camera::*;
use raytracing::colour::*;
use raytracing::hittable::*;
use raytracing::hittable_list::*;
use raytracing::ray::*;
use raytracing::rtweekend::*;
use raytracing::sphere::*;

use std::sync::Arc;

fn ray_colour(r: &Ray, world: &dyn Hittable) -> Vec3 {
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

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..image_width as i32 {
            let mut pixel_colour = Colour::new(0., 0., 0.);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand_f64()) / ((image_width - 1) as f64);
                let v = (j as f64 + rand_f64()) / ((image_height - 1) as f64);

                let r = cam.ray_at_offset(u, v);
                pixel_colour += ray_colour(&r, &world);
            }
            write_colour(&mut std::io::stdout(), pixel_colour, samples_per_pixel)?;
        }
    }

    eprintln!("\nDone!\n");
    Ok(())
}
