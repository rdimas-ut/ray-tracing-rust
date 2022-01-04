use std::rc::Rc;
use std::cell::RefCell;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

use crate::material::Material;

use crate::aabb::AABB;
use crate::aabb::surrounding_box;

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Rc<RefCell<dyn Material>>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0))*(self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center(r.time());
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
        let outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0: AABB = AABB {
            minimum: self.center(time0) - Vec3(self.radius, self.radius, self.radius),
            maximum: self.center(time0) + Vec3(self.radius, self.radius, self.radius)
        };

        let box1: AABB = AABB {
            minimum: self.center(time1) - Vec3(self.radius, self.radius, self.radius),
            maximum: self.center(time1) + Vec3(self.radius, self.radius, self.radius)
        };

        *output_box = surrounding_box(&box0, &box1);
        true
    }
}
