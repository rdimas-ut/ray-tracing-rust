use std::rc::Rc;
use std::cell::RefCell;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::hittable_first_week::Hittable;
use crate::hittable_first_week::HitRecord;
use crate::ray_first_week::Ray;

use crate::material_first_week::DefaultMaterial;
use crate::aabb_first_week::AABB;
use crate::aabb_first_week::surrounding_box;

pub struct HittableList{
    pub objects: Vec<Rc<RefCell<dyn Hittable>>>,
} 

impl HittableList {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<RefCell<dyn Hittable>>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord {
            p: Point3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            mat_ptr: Rc::new(RefCell::new(DefaultMaterial)),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object in self.objects.iter() {
            let j = object.borrow_mut().hit(r, t_min, closest_so_far, &mut temp_rec);
            if j {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box: AABB = AABB {
            minimum: Vec3(0.0, 0.0, 0.0),
            maximum: Vec3(0.0, 0.0, 0.0)
        };

        let mut first_box: bool = true;

        for object in self.objects.iter() {
            if !object.borrow().bounding_box(time0, time1, &mut temp_box) {return false;}
            *output_box = if first_box { temp_box } else { surrounding_box(output_box, &temp_box) };
            first_box = false;
        }

        true
    }
}