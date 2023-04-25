use raytracing::colour::*;
use raytracing::ray::*;
use raytracing::rtweekend::*;
use raytracing::hittable_list::*;
use raytracing::sphere::*;
use raytracing::hittable::*;

use std::sync::Arc;

fn ray_colour(r: &Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(rec) = world.hit(r, 0., INFTY) {
        return 0.5 * (rec.normal + Colour::new(1. ,1. ,1.));
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

    //World
    let mut world = HittableList::new_empty();
    world.add(Arc::new(Sphere { centre: Point3::new(0., 0., -1.), radius: 0.5 }));
    world.add(Arc::new(Sphere { centre: Point3::new(0., -100.5, -1.), radius: 100. }));

    // Camera
    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal * 0.5 - vertical * (0.5) - Vec3::new(0., 0., focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width as i32 {
            let u = i as f64 / (image_width as f64 - 1.);
            let v = j as f64 / (image_height as f64 - 1.);

            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };

            let colour = ray_colour(&r, &world);
            write_colour(&mut std::io::stdout(), colour)?;
        }
    }

    eprintln!("\nDone!\n");
    Ok(())
}
