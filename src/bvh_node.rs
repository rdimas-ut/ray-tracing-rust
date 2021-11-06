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
        let left: Rc<RefCell<dyn Hittable>>;
        let right: Rc<RefCell<dyn Hittable>>;
        
        let objects = src_objects.clone();

        let axis: i64 = rand::thread_rng().gen_range(0..2);
        let comparator = if axis == 0 { box_x_compare } 
                         if else axis == 1 { box_y_compare } 
                         else { box_z_compare }
        
        let object_span: u16 = end - start;

        if object_span == 1 {
            left = objects[start];
            right = objects[start];
        } else if object_span == 2 {
            if comparator(objects[start], objects[start+1]) {
                left = objects[start];
                right = objects[start+1];
            } else {
                left = objects[start+1];
                right = objects[start];
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span/2;
            left = Rc::new(RefCell::new(BvhNode.new(objects, start, mid, time0, time1)));
            right = Rc::new(RefCell::new(BvhNode.new(objects, mid, end, time0, time1)));
        }

        let box_left: AABB;
        let box_right: AABB;

        if !left.bounding_box(time0, time1, box_left) || !right.bounding_box(time0, time1, box_right) {
            println!("No bounding box in bvh_node constructor.");
        }

        BvhNode {
            left: left,
            right: right,
            box: surrounding_box(box_left, box_right),
        }
    }
}

fn box_compare(a: Rc<RefCell<dyn Hittable>>, b: Rc<RefCell<dyn Hittable>>, axis: u16) -> bool {
    let box_a: AABB;
    let box_b: AABB;
    
    if !a.borrow().bounding_box(0.0, box_a) || !b.borrow().bounding_box(0.0, box_b) {
        println!("No bounding box in bvh_node consctructor.");
    }

    box_a.min()[axis] < box_b.min()[axis]
}

fn box_x_compare(a: Rc<RefCell<dyn Hittable>>, b: Rc<RefCell<dyn Hittable>>) -> bool {
    box_compare(a, b, 0)
}

fn box_y_compare() -> bool {
    box_compare(a, b, 1)
}

fn box_z_compare() -> bool {
    box_compare(a, b, 2)
}