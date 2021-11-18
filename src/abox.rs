use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::hittable::HitRecord;
use crate::aabb::AABB;

use crate::ray::Ray;

use crate::material::Material;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::aarect;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ABox {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList
}

impl Hittable for ABox {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        return self.sides.hit(r, t_min, t_max, rec);
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB { 
            minimum: self.box_min, 
            maximum: self.box_max
        };
        return true;
    }
}

impl ABox {
    pub fn new(p0: &Point3, p1: &Point3, ptr: Rc<RefCell<Material>>) -> Self {
        let box_min = p0;
        let box_max = p1;

        let mut sides: HittableList = HittableList {objects: Vec::new() };
    
        sides.add(Rc::new(RefCell::new(aarect::XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), ptr.clone()))));
        sides.add(Rc::new(RefCell::new(aarect::XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), ptr.clone()))));
    
        sides.add(Rc::new(RefCell::new(aarect::XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), ptr.clone()))));
        sides.add(Rc::new(RefCell::new(aarect::XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), ptr.clone()))));
    
        sides.add(Rc::new(RefCell::new(aarect::YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), ptr.clone()))));
        sides.add(Rc::new(RefCell::new(aarect::YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), ptr.clone()))));
        
        ABox {
            box_min: box_min.clone(),
            box_max: box_max.clone(),
            sides: sides
        }
    }
}