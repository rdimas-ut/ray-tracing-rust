use crate::vec3::Vec3;
use crate::vec3::Point3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub tm: f64, 
}

impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        self.origin + t*self.direction
    }

    pub fn origin(self) -> Point3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn time(self) -> f64 {
        self.tm
    }
}