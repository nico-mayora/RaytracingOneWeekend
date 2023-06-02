use super::hittable::HitRecord;
use super::ray::Ray;
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

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction.normalize(), &rec.normal);
        let fuzzed = reflected + self.fuzz * rand_in_unit_sphere();
        let scattered = Ray {
            origin: rec.p,
            direction: fuzzed,
        };

        if scattered.direction.dot(&rec.normal) > 0. {
            return Some((self.albedo, scattered));
        } else {
            return None;
        }
    }
}

pub struct Dielectric {
    pub ir: f64, // Refraction index
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * f64::powi(1. - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let attenuation = Colour::new(1., 1., 1.);
        let unit_dir = r_in.direction.normalize();
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let cos_theta = f64::min((-unit_dir).dot(&rec.normal), 1.);
        let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            Vec3::reflect(&unit_dir, &rec.normal)
        } else {
            Vec3::refract(&unit_dir, &rec.normal, refraction_ratio)
        };

        let scattered = Ray {
            origin: rec.p,
            direction,
        };

        Some((attenuation, scattered))
    }
}
