use nalgebra::Vector3;
use rand::Rng;

pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;
pub type Colour = Vec3;

pub const INFTY: f64 = f64::MAX;
pub const PI: f64 = 3.1415926535897932385;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.
}

pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

// f64 doesn't have a total ordering (NaN).
pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}
