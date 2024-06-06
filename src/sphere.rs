use crate::material;

use super::hittable::*;
use super::material::Material;
use super::ray::*;
use super::rtweekend::*;
use std::sync::Arc;

pub struct Sphere {
    pub centre0: Point3, // Centre at time=0.
    pub displacement: Vec3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new_stationary(centre: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self { centre0: centre, displacement: Vec3::zeros(), radius, mat }
    }

    pub fn new_moving(centre0: Point3, centre1: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self { centre0, displacement: centre1 - centre0, radius, mat }
    }

    fn sphere_centre(&self, time: f64) -> Point3 {
        self.centre0 + time * self.displacement
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let centre = self.sphere_centre(r.time);

        let oc = r.origin - centre;
        let a = r.direction.norm_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - centre) / self.radius;

        Some(HitRecord::new(
            p,
            t,
            r,
            &outward_normal,
            Arc::clone(&self.mat),
        ))
    }
}
