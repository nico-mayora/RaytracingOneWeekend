use super::rtweekend::*;

pub trait Vec3RTExt {
    fn near_zero(&self) -> bool;
    fn reflect(&self, norm: Self) -> Self;
    fn mul(&self, x: Self) -> Self;
}

impl Vec3RTExt for Vec3 {
    fn near_zero(&self) -> bool {
        (self[0] < EPS) && (self[1] < EPS) && (self[2] < EPS)
    }

    fn reflect(&self, norm: Self) -> Self {
        self - 2. * self.dot(&norm) * norm
    }

    fn mul(&self, x: Self) -> Self {
        Self::new(self[0] * x[0], self[1] * x[1], self[2] * x[2])
    } 
}
