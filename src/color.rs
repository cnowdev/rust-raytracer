use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {
    let ir = (255.999 * pixel_color.x()) as u16;
    let ig = (255.999 * pixel_color.y()) as u16;
    let ib = (255.999 * pixel_color.z()) as u16;

    writeln!(out, "{} {} {}", ir, ig, ib)
}