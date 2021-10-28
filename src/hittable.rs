use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::ray::Ray;

use crate::material::Material;

use crate::aabb::AABB;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<RefCell<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}

pub trait Hittable {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}