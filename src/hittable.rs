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
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}

pub trait Hittable {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}

pub struct Translate {
    ptr: Rc<RefCell<dyn Hittable>>,
    offset: Vec3
}

impl Hittable for Translate {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r: Ray = Ray {
            origin: r.origin() - self.offset,
            direction: r.direction(),
            tm: r.time()
        };

        if !self.ptr.borrow_mut().hit(&moved_r, t_min, t_max, rec) {
            return false;
        };

        rec.p += self.offset;
        rec.set_face_normal(&moved_r, rec.normal);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.borrow().bounding_box(time0, time1, output_box) {
            return false;
        };

        *output_box = AABB {
            minimum: output_box.min() + self.offset,
            maximum: output_box.max() + self.offset,
        };

        return true;
    }
}

impl Translate {
    pub fn new(p: Rc<RefCell<dyn Hittable>>, displacement: Vec3) -> Self {
        Translate {
            ptr: p,
            offset: displacement
        }
    }
}

pub struct RotateY {
    ptr: Rc<RefCell<dyn Hittable>>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB
}

impl Hittable for RotateY {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta*r.origin()[0] - self.sin_theta*r.origin()[2];
        origin[2] = self.sin_theta*r.origin()[0] + self.cos_theta*r.origin()[2];

        direction[0] = self.cos_theta*r.direction()[0] - self.sin_theta*r.direction()[2];
        direction[2] = self.sin_theta*r.direction()[0] + self.cos_theta*r.direction()[2];
        
        let rotated_r: Ray = Ray {
            origin: origin,
            direction: direction,
            tm: r.time()
        };

        if !self.ptr.borrow_mut().hit(&rotated_r, t_min, t_max, rec) {
            return false;
        };

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] =   self.cos_theta*rec.p[0] +  self.sin_theta*rec.p[2];
        p[2] = - self.sin_theta*rec.p[0] +  self.cos_theta*rec.p[2];

        normal[0] =   self.cos_theta*rec.normal[0] +  self.sin_theta*rec.normal[2];
        normal[2] = - self.sin_theta*rec.normal[0] +  self.cos_theta*rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);

        return true;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        return self.hasbox;
    }
}

impl RotateY {
    pub fn new(p: Rc<RefCell<dyn Hittable>>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox: AABB = AABB { 
            minimum: Point3(0.0, 0.0, 0.0),
            maximum: Point3(0.0, 0.0, 0.0),
         };
        let hasbox = p.borrow().bounding_box(0.0, 1.0, &mut bbox);

        let mut min = Point3( std::f64::MAX,  std::f64::MAX,  std::f64::MAX);
        let mut max = Point3(-std::f64::MAX, -std::f64::MAX, -std::f64::MAX);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64*bbox.max().x() + (1.0 - i as f64)*bbox.min().x();
                    let y = j as f64*bbox.max().y() + (1.0 - j as f64)*bbox.min().y();
                    let z = k as f64*bbox.max().z() + (1.0 - k as f64)*bbox.min().z();

                    let newx =  cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = AABB {
            minimum: min, 
            maximum: max
        };

        RotateY {
            ptr: p.clone(),
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            hasbox: hasbox,
            bbox: bbox,
        }        
    }
}