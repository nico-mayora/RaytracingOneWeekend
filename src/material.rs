use super::ray::Ray;
use super::hittable::HitRecord;
use super::rtweekend::*;
use super::vec3rtext::*;

pub trait Material {
    // None means the ray was abosorbed
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)>;
}

pub struct Lambertian {
    pub albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal + rand_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray { origin: rec.p, direction: scatter_direction };
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Colour,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = r_in.direction.normalize().reflect(rec.normal);
        let scattered = Ray { origin: rec.p, direction: reflected };

        if scattered.direction.dot(&rec.normal) > 0. {
            return Some((self.albedo, scattered));
        } else {
            return None;
        }
    }
}
