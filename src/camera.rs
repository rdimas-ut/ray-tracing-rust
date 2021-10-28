use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::vec3::degrees_to_radians;
use crate::vec3::random_in_unit_disk;
use crate::ray::Ray;

use crate::rtweekend::random_double;

use rand::distributions::Uniform;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = self.u*rd.x() + self.v*rd.y();
        let random_dist = Uniform::new(self.time0, self.time1);

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
            tm: random_double(random_dist)
        }
    }

    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, _time0: f64, _time1: f64) -> Self  {
            let theta: f64 = degrees_to_radians(vfov);
            let h = (theta/2.0).tan();
            let ASPECT_RATIO: f64 = aspect_ratio;
            let VIEWPORT_HEIGHT: f64 = 2.0 * h;
            let VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;

            let w = Vec3::unit_vector(lookfrom - lookat);
            let u: Vec3 = Vec3::unit_vector(Vec3::cross(vup, w));
            let v: Vec3 = Vec3::cross(w, u);


            let hor: Vec3 = focus_dist * VIEWPORT_WIDTH * u;
            let ver: Vec3 = focus_dist * VIEWPORT_HEIGHT * v;
            
            Camera {
                origin: lookfrom,
                horizontal: hor,
                vertical: ver,
                lower_left_corner: lookfrom - hor/2.0 - ver/2.0 - focus_dist*w,
                u: u,
                v: v,
                w: w,
                lens_radius: aperture/2.0,
                time0: _time0,
                time1: _time1,
            }
    }
}