use super::material::Material;
use super::ray::*;
use crate::util::rtweekend::*;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, r: &Ray, outward_normal: &Vec3, mat: Arc<dyn Material>) -> Self {
        let fields = Self::calculate_face_normal(r, outward_normal);
        HitRecord {
            p,
            t,
            normal: fields.1,
            front_face: fields.0,
            mat,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        let fields = Self::calculate_face_normal(r, outward_normal);
        self.front_face = fields.0;
        self.normal = fields.1;
    }

    fn calculate_face_normal(r: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        let front_face = r.direction.dot(outward_normal) < 0.;
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
