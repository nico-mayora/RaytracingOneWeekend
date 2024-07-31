use crate::util::rtweekend::Vec3;

pub struct Settings {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_distance: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub width: u32,
}

// TODO: read from file
impl Default for Settings {
    fn default() -> Self {
        Self {
            lookfrom: [13.0, 2.0, 3.0].into(),
            lookat: [0.0, 0.0, 0.0].into(),
            vup: [ 0.0, 1.0, 0.0 ].into(),
            vfov: 20.0,
            aspect_ratio: 1.5,
            aperture: 0.1,
            focus_distance: 10.0,
            samples_per_pixel: 50,
            max_depth: 30,
            width: 800,
        }
    }
}
