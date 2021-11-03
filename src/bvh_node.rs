use std::rc::Rc;
use std::cell::RefCell;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use rand::Rng;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

use crate::material::DefaultMaterial;
use crate::aabb::AABB;
use crate::aabb::surrounding_box;


pub struct BvhNode {
    pub left: Rc<RefCell<dyn Hittable>>,
    pub right: Rc<RefCell<dyn Hittable>>,
    pub box: AABB
}

impl Hittable for BvhNode {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.box.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left: bool = self.left.borrow().hit(r, t_min, t_max, rec);
        let hit_right: bool = self.right.borrow().hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.box;
        true
    } 
}

impl BvhNode {
    fn new(src_objects: Vec<Rc<RefCell<dyn Hittable>>>, start: u16, end:u16 time0: f64, time1: f64) -> Self {
        let objects = src_objects.clone();

        let axis: i64 = rand::thread_rng().gen_range(0..3);
        let comparator = if axis == 0 { }
    }
}