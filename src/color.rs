use crate::vec3::Vec3;
use std::io::Write;
use crate::interval::Interval;

pub type Color = Vec3;

const INTENSITY: Interval = Interval::new(0.0, 0.999);

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { linear_component.sqrt() } else { 0.0 }
}


pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {
    
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let ir = (256.0 * INTENSITY.clamp(r)) as u16;
    let ig = (256.0 * INTENSITY.clamp(g)) as u16;
    let ib = (256.0 * INTENSITY.clamp(b)) as u16;


    writeln!(out, "{} {} {}", ir, ig, ib)
}