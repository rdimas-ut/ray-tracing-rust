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
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin 
        }
    }

    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self  {
            let theta: f64 = degrees_to_radians(vfov);
            let h = (theta/2.0).tan();
            let ASPECT_RATIO: f64 = aspect_ratio;
            let VIEWPORT_HEIGHT: f64 = 2.0 * h;
            let VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
            let FOCAL_LENGTH: f64 = 1.0;

            let w = Vec3::unit_vector(lookfrom - lookat);
            let u: Vec3 = Vec3::unit_vector(Vec3::cross(vup, w));
            let v: Vec3 = Vec3::cross(w, u);


            let hor: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
            let ver: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
            
            Camera {
                origin: lookfrom,
                horizontal: VIEWPORT_WIDTH * u,
                vertical: VIEWPORT_HEIGHT * v,
                lower_left_corner: lookfrom - hor/2.0 - ver/2.0 - w
            }
    }
}