pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
use rand;


#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    // random double [0, 1)
    return rand::random::<f64>()
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // random double [min, max)
    return min + (max - min) * random_double()
}