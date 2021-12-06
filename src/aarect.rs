use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::aabb::AABB;

use crate::ray::Ray;

use crate::material::Material;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use std::rc::Rc;
use std::cell::RefCell;

pub struct XYRect {
    mp: Rc<RefCell<dyn Material>>,
    x0: f64, 
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl Hittable for XYRect {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();

        if (t < t_min) || (t > t_max) {
            return false;
        }

        let x = r.origin().x() + t*r.direction().x();
        let y = r.origin().y() + t*r.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x-self.x0)/(self.x1-self.x0);
        rec.v = (y-self.y0)/(self.y1-self.y0);
        rec.t = t;
        let outward_normal = Vec3(0.0, 0.0, 1.0);
        rec.set_face_normal(*r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return true;

    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        // The bounding box must have non-zero width in each dimension, so the z
        // dimension is padded a small amount

        *output_box = AABB {
            minimum: Point3(self.x0, self.y0, self.k - 0.0001),
            maximum: Point3(self.x1, self.y1, self.k + 0.0001)
        };
        return true;
    }
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Rc<RefCell<dyn Material>>) -> Self {
        XYRect {
            mp: mp,
            x0: x0, 
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
        }
    }
}

pub struct XZRect {
    mp: Rc<RefCell<dyn Material>>,
    x0: f64, 
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl Hittable for XZRect {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();

        if (t < t_min) || (t > t_max) {
            return false;
        }

        let x = r.origin().x() + t*r.direction().x();
        let z = r.origin().z() + t*r.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x-self.x0)/(self.x1-self.x0);
        rec.v = (z-self.z0)/(self.z1-self.z0);
        rec.t = t;
        let outward_normal = Vec3(0.0, 1.0, 0.0);
        rec.set_face_normal(*r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return true;

    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        // The bounding box must have non-zero width in each dimension, so the z
        // dimension is padded a small amount

        *output_box = AABB {
            minimum: Point3(self.x0, self.k - 0.0001, self.z0),
            maximum: Point3(self.x1, self.k + 0.0001, self.z1)
        };
        return true;
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Rc<RefCell<dyn Material>>) -> Self {
        XZRect {
            mp: mp,
            x0: x0, 
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
        }
    }
}

pub struct YZRect {
    mp: Rc<RefCell<dyn Material>>,
    y0: f64, 
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl Hittable for YZRect {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();

        if (t < t_min) || (t > t_max) {
            return false;
        }

        let y = r.origin().y() + t*r.direction().y();
        let z = r.origin().z() + t*r.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y-self.y0)/(self.y1-self.y0);
        rec.v = (z-self.z0)/(self.z1-self.z0);
        rec.t = t;
        let outward_normal = Vec3(1.0, 0.0, 0.0);
        rec.set_face_normal(*r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return true;

    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        // The bounding box must have non-zero width in each dimension, so the z
        // dimension is padded a small amount

        *output_box = AABB {
            minimum: Point3(self.k - 0.0001, self.y0, self.z0),
            maximum: Point3(self.k + 0.0001, self.y1, self.z1)
        };
        return true;
    }
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Rc<RefCell<dyn Material>>) -> Self {
        YZRect {
            mp: mp,
            y0: y0, 
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
        }
    }
}