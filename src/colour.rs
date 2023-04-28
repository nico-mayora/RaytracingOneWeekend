use super::rtweekend::*;
use std::io::Write;

pub fn write_colour<W: Write>(out: &mut W, pixel_colour: Colour, samples_per_pixel: i32) -> std::io::Result<()> {
    let mut r = pixel_colour[0];
    let mut g = pixel_colour[1];
    let mut b = pixel_colour[2];

    let scale = 1. / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    out.write_all(
        format!("{} {} {}\n", 
                (256. * clamp(r, 0., 0.999)) as i32,
                (256. * clamp(g, 0., 0.999)) as i32,
                (256. * clamp(b, 0., 0.999)) as i32).as_bytes()
    )?;

    Ok(())
}
