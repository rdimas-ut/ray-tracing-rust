use crate::vec3::Color;
use crate::vec3::Point3;

trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

struct SolidColor {
    color_value: Color;
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value
    }
}