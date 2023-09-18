use std::rc::Rc;
use std::cell::RefCell;
use std::f64::consts::PI;

use crate::onb::Onb;
use crate::rtweekend::random_double;
use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

use crate::material::Material;
use crate::material::DefaultMaterial;

use crate::aabb::AABB;


#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<RefCell<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_square();
        let half_b: f64 = Vec3::dot(oc, r.direction());
        let c: f64 = oc.length_square() - self.radius*self.radius;

        let discriminant: f64 = half_b*half_b - a*c;
        if discriminant < 0.0 {return false;}
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        self.get_sphere(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB{
            minimum: self.center - Vec3(self.radius, self.radius, self.radius), 
            maximum: self.center + Vec3(self.radius, self.radius, self.radius),
        };
        true
    }

    fn pdf_value(&mut self, o: &Vec3, v: &Vec3) -> f64 {
        let mut rec: HitRecord = HitRecord {
            p: Point3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            mat_ptr: Rc::new(RefCell::new(DefaultMaterial)),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };

        if !self.hit(&Ray {origin: *o, direction: *v, tm: 0.0}, 0.001, f64::INFINITY, &mut rec) {
            return 0.0;
        }

        let cos_theta_max = (1.0 - self.radius*self.radius/(self.center-*o).length_square()).sqrt();
        let solid_angle = 2.0*PI*(1.0 - cos_theta_max);

        return 1.0/solid_angle;
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let direction: Vec3 = self.center - *o;
        let distance_squared = direction.length_square();
        let mut uvw: Onb = Onb(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0));
        uvw.build_from_w(&direction);
        uvw.local_vec(&Sphere::random_to_sphere(self.radius, distance_squared))
    }
}

impl Sphere {
    fn get_sphere(&self, p: &Point3, u: &mut f64, v: &mut f64) -> () {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        *u = phi / (2.0*PI);
        *v = theta / PI;
    }

    fn random_to_sphere(radius: f64,  distance_squared: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2*((1.0-radius*radius/distance_squared).sqrt() - 1.0);

        let phi = 2.0*PI*r1;
        let x = phi.cos()*(1.0-z*z).sqrt();
        let y = phi.sin()*(1.0-z*z).sqrt();

        return Vec3(x, y, z);
    }
}