use crate::vec3::{Vec3, random_unit_vector, random_cosine_direction};
use crate::onb::Onb;

use std::f64::consts::PI;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

struct SpherePdf();

impl Pdf for SpherePdf {

    fn value(&self, _direction: &Vec3) -> f64 {
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
    fn value(&self, direction: &Vec3) -> f64 {
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

