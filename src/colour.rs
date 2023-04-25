use super::rtweekend::*;
use std::io::Write;

pub fn write_colour<W: Write>(out: &mut W, pixel_colour: Colour) -> std::io::Result<()> {
    let r = (255.999 * pixel_colour[0]) as i32;
    let g = (255.999 * pixel_colour[1]) as i32;
    let b = (255.999 * pixel_colour[2]) as i32;

    out.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;

    Ok(())
}
