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
        let hit_left: bool = self.left.borrow_mut().hit(r, t_min, t_max, rec);
        let hit_right: bool = self.right.borrow_mut().hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
    
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.abox.clone();
        true
    } 
}

impl BvhNode {
    pub fn new(src_objects: &mut Vec<Rc<RefCell<dyn Hittable>>>, start: usize, end:usize, time0: f64, time1: f64) -> Self {
        let left: Rc<RefCell<dyn Hittable>>;
        let right: Rc<RefCell<dyn Hittable>>;
        
        let mut objects = src_objects;

        let axis: i64 = rand::thread_rng().gen_range(0..3);
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
        
        let object_span: usize = end - start;

        if object_span == 1 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start]);
            eprintln!("left: {}, right: {}", start, start);
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start+1]) == std::cmp::Ordering::Less {
                left = Rc::clone(&objects[start]);
                right = Rc::clone(&objects[start+1]);
                eprintln!("left: {}, right: {}", start, start+1);
            } else {
                left = Rc::clone(&objects[start+1]);
                right = Rc::clone(&objects[start]);
                eprintln!("left: {}, right: {}", start+1, start);
            }
        } else {
            objects[start..end].sort_unstable_by(comparator);

            let mid: usize = start + (object_span/2);
            left = Rc::new(RefCell::new(BvhNode::new(&mut objects, start, mid, time0, time1)));
            right = Rc::new(RefCell::new(BvhNode::new(&mut objects, mid, end, time0, time1)));
            eprintln!("left: {}, {} right: {}, {}", start, mid, mid, end);
        }

        let mut box_left: AABB = AABB {
            minimum: Point3(0.0, 0.0, 0.0),
            maximum: Point3(0.0, 0.0, 0.0)
        };
        let mut box_right: AABB = AABB {
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

fn box_compare(a: &Rc<RefCell<dyn Hittable>>, b: &Rc<RefCell<dyn Hittable>>, axis: u32) -> std::cmp::Ordering {
    let mut box_a: AABB = AABB {
        minimum: Point3(0.0, 0.0, 0.0),
        maximum: Point3(0.0, 0.0, 0.0)
    };
    let mut box_b: AABB= AABB {
        minimum: Point3(0.0, 0.0, 0.0),
        maximum: Point3(0.0, 0.0, 0.0)
    };
    
    if !a.borrow_mut().bounding_box(0.0, 0.0, &mut box_a) || !b.borrow_mut().bounding_box(0.0, 0.0,&mut box_b) {
        println!("No bounding box in bvh_node consctructor.");
    }
    box_a.min()[axis].partial_cmp(&box_b.min()[axis]).expect("unexpected NaN in bounding box y")

    // if box_a.min()[axis] < box_b.min()[axis] {
    //     return std::cmp::Ordering::Less;
    // } else if box_a.max()[axis] > box_b.max()[axis] {
    //     return std::cmp::Ordering::Greater;
    // } else {
    //     return std::cmp::Ordering::Equal;
    // }
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