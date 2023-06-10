use super::hittable::*;
use super::ray::*;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new_empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from_hittable(object: Arc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t;
                temp_rec = Some(hit_rec);
            }
        }

        return temp_rec;
    }
}

unsafe impl Sync for HittableList {}
