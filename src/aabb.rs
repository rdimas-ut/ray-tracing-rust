use crate::vec3::Point3;
use crate::Ray;

#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3
}

impl AABB {
    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max:f64) -> bool {
        for a in 0..3 {
            let t0 = (self.minimum[a] - r.origin()[a] / r.direction()[a]).min(self.maximum[a] - r.origin()[a] / r.direction()[a]);
            let t1 = (self.minimum[a] - r.origin()[a] / r.direction()[a]).max(self.maximum[a] - r.origin()[a] / r.direction()[a]);
            
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            } 
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small: Point3 = Point3(box0.min().x().min(box1.min().x()), 
                               box0.min().y().min(box1.min().y()), 
                               box0.min().z().min(box1.min().z()));

    let big: Point3 = Point3(box0.max().x().max(box1.max().x()), 
                            box0.max().y().max(box1.max().y()), 
                            box0.max().z().max(box1.max().z()));

    return AABB{minimum: small, maximum: big};
}