use crate::hittable::Hittable;
use crate::rtweekend::random_double;
use crate::vec3::{Vec3, Point3, random_unit_vector, random_cosine_direction};
use crate::onb::Onb;

use std::f64::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Pdf {
    fn value(&mut self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

struct SpherePdf();

impl Pdf for SpherePdf {

    fn value(&mut self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePdf {
    pub uvw: Onb
}

impl Pdf for CosinePdf {
    fn value(&mut self, direction: &Vec3) -> f64 {
        let cosine_theta = Vec3::dot(Vec3::unit_vector(*direction), self.uvw.w());
        (0.0_f64).max(cosine_theta/PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        let mut x = Onb(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0));
        x.build_from_w(&w);
        CosinePdf {uvw: x}
    }
}

pub struct HittablePdf {
    pub objects:  Rc<RefCell<dyn Hittable>>,
    pub origin: Point3,
}

impl Pdf for HittablePdf {
    fn value(&mut self, direction: &Vec3) -> f64 {
        self.objects.borrow_mut().pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.borrow().random(&self.origin)
    }
}

pub struct MixturePdf(pub Rc<RefCell<dyn Pdf>>, pub Rc<RefCell<dyn Pdf>>);

impl Pdf for MixturePdf {
    fn value(&mut self, direction: &Vec3) -> f64 {
        0.5 * self.0.borrow_mut().value(direction) + 0.5 * self.1.borrow_mut().value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            return self.0.borrow().generate();
        } else {
            return self.1.borrow().generate();
        }
    }
}