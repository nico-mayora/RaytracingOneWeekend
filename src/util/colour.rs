use super::rtweekend::*;

use image::RgbImage;
use num::clamp;
use std::io::Write;

pub fn drawn_colour(pixel_colour: Colour, samples_per_pixel: u32) -> [u8; 3] {
    let mut r = pixel_colour[0];
    let mut g = pixel_colour[1];
    let mut b = pixel_colour[2];

    let scale = 1. / samples_per_pixel as f64;
    r = f64::sqrt(r * scale);
    g = f64::sqrt(g * scale);
    b = f64::sqrt(b * scale);

    [
        (256. * clamp(r, 0., 0.999)) as u8,
        (256. * clamp(g, 0., 0.999)) as u8,
        (256. * clamp(b, 0., 0.999)) as u8,
    ]
}

pub fn drawn_colour_with_alpha(pixel_colour: Colour, samples_per_pixel: u32) -> [u8; 4] {
    let rgb = drawn_colour(pixel_colour, samples_per_pixel);
    [rgb[0], rgb[1], rgb[2], 0xFF]
}

// For PPM format
pub fn write_colour<W: Write>(
    out: &mut W,
    pixel_colour: Colour,
    samples_per_pixel: u32,
) -> std::io::Result<()> {
    let rgb = drawn_colour(pixel_colour, samples_per_pixel);

    out.write_all(
        format!(
            "{} {} {}\n",
            rgb[0] as i32,
            rgb[1] as i32,
            rgb[2] as i32
        )
        .as_bytes(),
    )?;

    Ok(())
}

// for PNG
pub fn write_to_img(
    img: &mut RgbImage,
    pixel_colour: Colour,
    samples_per_pixel: u32,
    posx: i32,
    posy: i32,
) {
    let rgb = drawn_colour(pixel_colour, samples_per_pixel);

    *img.get_pixel_mut(posx as u32, posy as u32) = image::Rgb([
        rgb[0] as u8,
        rgb[1] as u8,
        rgb[2] as u8,
    ]);
}
