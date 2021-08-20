use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        {
            const ASPECT_RATIO: f64 = 16.0 / 9.0;
            const VIEWPORT_HEIGHT: f64 = 2.0;
            const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
            const FOCAL_LENGTH: f64 = 1.0;

            let hor: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
            let ver: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
            
            Camera {
                origin: Point3(0.0, 0.0, 0.0),
                horizontal: Vec3(VIEWPORT_WIDTH, 0.0, 0.0),
                vertical: Vec3(0.0, VIEWPORT_HEIGHT, 0.0),
                lower_left_corner: Point3(0.0, 0.0, 0.0) - hor/2.0 - ver/2.0 - Vec3(0.0, 0.0, FOCAL_LENGTH)
            }
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin 
        }
    }
}