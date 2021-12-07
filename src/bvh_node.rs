use std::rc::Rc;
use std::cell::RefCell;

use crate::vec3::Point3;

use rand::Rng;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

use crate::aabb::AABB;
use crate::aabb::surrounding_box;


pub struct BvhNode {
    pub left: Rc<RefCell<dyn Hittable>>,
    pub right: Rc<RefCell<dyn Hittable>>,
    pub abox: AABB
}

impl Hittable for BvhNode {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.abox.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left: bool = self.left.borrow_mut().hit(r, t_min, t_max, rec);
        let hit_right: bool = self.right.borrow_mut().hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.abox;
        true
    } 
}

impl BvhNode {
    pub fn new(src_objects: Vec<Rc<RefCell<dyn Hittable>>>, start: u16, end:u16, time0: f64, time1: f64) -> Self {
        let left: Rc<RefCell<dyn Hittable>>;
        let right: Rc<RefCell<dyn Hittable>>;
        
        let mut objects = src_objects;

        let axis: i64 = rand::thread_rng().gen_range(0..2);
        let comparator: &dyn Fn(&Rc<RefCell<dyn Hittable>>, &Rc<RefCell<dyn Hittable>>) -> std::cmp::Ordering;
        if axis == 0 { 
            comparator = &box_x_compare; 
        } 
        else if axis == 1 { 
            comparator = &box_y_compare;
        } 
        else { 
            comparator = &box_z_compare;
        }
        
        let object_span: u16 = end - start;

        if object_span == 1 {
            left = objects[start as usize].clone();
            right = objects[start as usize].clone();
        } else if object_span == 2 {
            if comparator(&objects[start as usize], &objects[start as usize+1]) == std::cmp::Ordering::Less {
                left = objects[start as usize].clone();
                right = objects[start as usize+1].clone();
            } else {
                left = objects[start as usize+1].clone();
                right = objects[start as usize].clone();
            }
        } else {
            objects[start as usize..end as usize].sort_by(box_x_compare);

            let mid = start + object_span/2;
            left = Rc::new(RefCell::new(BvhNode::new(objects.clone(), start, mid, time0, time1)));
            right = Rc::new(RefCell::new(BvhNode::new(objects, mid, end, time0, time1)));
        }

        let mut box_left: AABB = AABB {
            minimum: Point3(0.0, 0.0, 0.0),
            maximum: Point3(0.0, 0.0, 0.0)
        };
        let mut box_right: AABB= AABB {
            minimum: Point3(0.0, 0.0, 0.0),
            maximum: Point3(0.0, 0.0, 0.0)
        };

        if !left.borrow().bounding_box(time0, time1, &mut box_left) || !right.borrow().bounding_box(time0, time1,  &mut box_right) {
            println!("No bounding box in bvh_node constructor.");
        }

        BvhNode {
            left: left,
            right: right,
            abox: surrounding_box(&box_left, &box_right),
        }
    }
}

fn box_compare(a: &Rc<RefCell<dyn Hittable>>, b: &Rc<RefCell<dyn Hittable>>, axis: u16) -> std::cmp::Ordering {
    let mut box_a: AABB = AABB {
        minimum: Point3(0.0, 0.0, 0.0),
        maximum: Point3(0.0, 0.0, 0.0)
    };
    let mut box_b: AABB= AABB {
        minimum: Point3(0.0, 0.0, 0.0),
        maximum: Point3(0.0, 0.0, 0.0)
    };
    
    if !a.borrow().bounding_box(0.0, 0.0, &mut box_a) || !b.borrow().bounding_box(0.0, 0.0,&mut box_b) {
        println!("No bounding box in bvh_node consctructor.");
    }

    if box_a.min()[axis as u32] < box_b.min()[axis as u32] {
        return std::cmp::Ordering::Less;
    } else if box_a.min()[axis as u32] > box_b.min()[axis as u32] {
        return std::cmp::Ordering::Greater;
    } else {
        return std::cmp::Ordering::Equal;
    }
}

fn box_x_compare(a: &Rc<RefCell<dyn Hittable>>, b: &Rc<RefCell<dyn Hittable>>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Rc<RefCell<dyn Hittable>>, b: &Rc<RefCell<dyn Hittable>>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Rc<RefCell<dyn Hittable>>, b: &Rc<RefCell<dyn Hittable>>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}
