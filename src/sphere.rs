use std::rc::Rc;
use std::cell::RefCell;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

use crate::material::Material;

use crate::aabb::AABB;


#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<RefCell<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_square();
        let half_b: f64 = Vec3::dot(oc, r.direction());
        let c: f64 = oc.length_square() - self.radius*self.radius;

        let discriminant: f64 = half_b*half_b - a*c;
        if discriminant < 0.0 {return false;}
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(*r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB{
            minimum: self.center - Vec3(self.radius, self.radius, self.radius), 
            maximum: self.center + Vec3(self.radius, self.radius, self.radius),
        };
        true
    }
}