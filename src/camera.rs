use super::rtweekend::*;
use super::ray::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16./9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner = origin - horizontal*0.5 - vertical*0.5 - Vec3::new(0., 0., focal_length);

        Self { origin, horizontal, vertical, lower_left_corner }
    }

    pub fn ray_at_offset(&self, u: f64, v: f64) -> Ray {
        Ray { origin: self.origin , direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin}
    }
}
