use super::ray::Ray;
use super::hittable::HitRecord;
use super::rtweekend::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Colour) -> Option<Ray>;
}

struct Lambertian {
    pub albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Colour) -> bool {
        let scatter_direction = rec.normal + rand_unit_vector();
        let scattered = Ray { origin: rec.p, direction: scatter_direction };
        attenuation
    }
}

