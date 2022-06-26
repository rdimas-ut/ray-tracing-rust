use crate::vec3::Vec3;
use std::ops::Index;
use std::ops::IndexMut;

pub struct Onb(pub Vec3, pub Vec3, pub Vec3);


impl Onb {
    pub fn u(&self) -> Vec3 {
        self.0
    }

    pub fn v(&self) -> Vec3 {
        self.1
    }

    pub fn w(&self) -> Vec3 {
        self.2
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a*self.u() + b*self.v() + c*self.w()
    }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        a.x()*self.u() + a.y()*self.v() + a.z()*self.w()
    }

    pub fn build_from_w(&mut self, w: &Vec3) {
        let unit_w: Vec3 = Vec3::unit_vector(*w);
        let a: Vec3 = if unit_w.x().abs() > 0.9 {Vec3(0.0, 1.0, 0.0)} else {Vec3(1.0, 0.0, 0.0)};
        let v: Vec3 = Vec3::unit_vector(Vec3::cross(unit_w, a));
        let u: Vec3 = Vec3::cross(unit_w, v);
        self.0 = u;
        self.1 = v;
        self.2 = unit_w; 
    }
}

impl Index<u32> for Onb {
    type Output = Vec3;

    fn index(&self, i: u32) -> &Vec3 {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => &self.2,
        }
    }
}

impl IndexMut<u32> for Onb {
    fn index_mut(&mut self, i: u32) -> &mut Vec3{
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => &mut self.2,
        }
    }
}