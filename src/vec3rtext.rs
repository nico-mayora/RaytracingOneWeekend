use super::rtweekend::*;

// Extensions to nalgebra's Vector implementation for use in this project.
pub trait Vec3RTExt {
    fn near_zero(&self) -> bool;
    fn mul(&self, x: &Self) -> Self;
    fn reflect(uv: &Self, norm: &Self) -> Self;
    fn refract(uv: &Self, norm: &Self, etai_over_etat: f64) -> Self;
}

impl Vec3RTExt for Vec3 {
    fn near_zero(&self) -> bool {
        (self[0].abs() < EPS) && (self[1].abs() < EPS) && (self[2].abs() < EPS)
    }

    fn mul(&self, x: &Self) -> Self {
        Self::new(self[0] * x[0], self[1] * x[1], self[2] * x[2])
    }

    fn reflect(uv: &Self, norm: &Self) -> Self {
        uv - 2. * uv.dot(norm) * norm
    }

    fn refract(uv: &Self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min((-uv).dot(n), 1.);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1. - r_out_perp.norm_squared())) * n;
        r_out_perp + r_out_parallel
    }
}
