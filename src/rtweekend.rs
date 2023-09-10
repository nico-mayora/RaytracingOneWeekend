use nalgebra::Vector3;
use rand::*;

pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;
pub type Colour = Vec3;

// pub const INFTY: f64 = f64::MAX;
pub const EPS: f64 = 1e-8;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(rand::random(), rand::random(), rand::random())
}

pub fn random_vec3_range(min: f64, max: f64) -> Vec3 {
    let mut rng = thread_rng();

    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn rand_in_unit_disk() -> Vec3 {
    loop {
        let mut rng = rand::thread_rng();

        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.norm_squared() < 1. {
            return p;
        }
    }
}

pub fn rand_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_range(-1., 1.);
        if p.norm_squared() < 1. {
            return p;
        }
    }
}

pub fn rand_unit_vector() -> Vec3 {
    rand_in_unit_sphere().normalize()
}

pub fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = rand_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}
