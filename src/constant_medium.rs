use crate::vec3::Vec3;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::random_double;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;

use crate::material::Material;
use crate::material::DefaultMaterial;
use crate::material::Isotropic;

use crate::aabb::AABB;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ConstantMedium {
    boundary: Rc<RefCell<dyn Hittable>>,
    phase_function: Rc<RefCell<dyn Material>>,
    neg_inv_density: f64
}

impl Hittable for ConstantMedium {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enable_debug true.
        let enable_debug: bool = false;
        let debugging: bool = enable_debug && random_double() < 0.00001;

        let mut rec1: HitRecord = HitRecord {
            p: Point3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            mat_ptr: Rc::new(RefCell::new(DefaultMaterial)),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };

        let mut rec2: HitRecord = HitRecord {
            p: Point3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            mat_ptr: Rc::new(RefCell::new(DefaultMaterial)),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };

        if !self.boundary.borrow_mut().hit(r, -std::f64::MAX, std::f64::MAX, &mut rec1) {
            return false;
        }

        if !self.boundary.borrow_mut().hit(r, rec1.t + 0.0001, std::f64::MAX, &mut rec2) {
            return false;
        }

        if debugging {
            eprintln!("t_min={}, t_max={}", rec1.t, rec2.t);
        } 
        
        if rec1.t < t_min { rec1.t = t_min; }
        if rec2.t < t_max { rec2.t = t_max; }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.p);
        }

        rec.normal = Vec3(1.0, 0.0, 0.0);  // arbitrary
        rec.front_face = true;     // also arbitrary
        rec.mat_ptr = self.phase_function.clone();

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        return self.boundary.borrow().bounding_box(time0, time1, output_box);
    }
}

impl ConstantMedium {
    pub fn new(b: Rc<RefCell<dyn Hittable>>, d: f64, c: Color) -> Self {
        ConstantMedium {
            boundary: b,
            phase_function: Rc::new(RefCell::new(Isotropic::new(c))),
            neg_inv_density: -1.0/d,
        }
    }
}