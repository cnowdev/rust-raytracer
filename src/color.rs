use crate::vec3::Vec3;
use std::io::Write;
use crate::interval::Interval;

pub type Color = Vec3;

const INTENSITY: Interval = Interval::new(0.0, 0.999);
pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {
    
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let ir = (256.0 * INTENSITY.clamp(r)) as u16;
    let ig = (256.0 * INTENSITY.clamp(g)) as u16;
    let ib = (256.0 * INTENSITY.clamp(b)) as u16;


    writeln!(out, "{} {} {}", ir, ig, ib)
}