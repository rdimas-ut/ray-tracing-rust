use crate::vec3::Color;

use crate::rtweekend::clamp;

pub fn write_color(pixel_color: Color, samples_per_pixel: u64) {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    // Divide the color by the number of samples
    let scale: f64 = 1.0/samples_per_pixel as f64;

    r *= scale; r = r.sqrt();
    g *= scale; g = g.sqrt();
    b *= scale; b = b.sqrt();

    println!("{} {} {}", 
        (256.0 * clamp(r, 0.0, 0.999)) as u64, 
        (256.0 * clamp(g, 0.0, 0.999)) as u64,
        (256.0 * clamp(b, 0.0, 0.999)) as u64)
}