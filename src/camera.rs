use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::vec3::degrees_to_radians;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin 
        }
    }

    pub fn new(vfov: f64, aspect_ratio: f64) -> Self  {
            let theta: f64 = degrees_to_radians(vfov);
            let h = (theta/2.0).tan();
            let ASPECT_RATIO: f64 = aspect_ratio;
            let VIEWPORT_HEIGHT: f64 = 2.0 * h;
            let VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
            let FOCAL_LENGTH: f64 = 1.0;

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