use nalgebra::Vector3;

pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;
pub type Colour = Vec3;

pub const INFTY: f64 = f64::MAX;
pub const PI: f64 = 3.1415926535897932385;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.
}
