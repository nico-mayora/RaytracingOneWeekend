use super::rtweekend::*;

use image::RgbImage;
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

pub fn write_to_img(img: &mut RgbImage, pixel_colour: Colour, samples_per_pixel: i32, posx: i32, posy: i32) {
    let mut r = pixel_colour[0];
    let mut g = pixel_colour[1];
    let mut b = pixel_colour[2];

    let scale = 1. / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    *img.get_pixel_mut(posx as u32, posy as u32) = image::Rgb([(256. * clamp(r, 0., 0.999)) as u8,
                                           (256. * clamp(g, 0., 0.999)) as u8,
                                           (256. * clamp(b, 0., 0.999)) as u8,]);
}
