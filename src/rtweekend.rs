use rand::random;
use std::f64::consts::PI;

// Returns a random f64(real) number in from [0, 1)
pub fn random_double() -> f64 {
    random::<f64>()
}

// Returns a random f64(real) number in [min,max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min)*random_double()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees*PI)/180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min };
    if x > max { return max };
    return x;
}