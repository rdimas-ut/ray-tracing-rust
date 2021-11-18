use crate::vec3::Vec3;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::random_unit_vector;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::refract;

use crate::ray::Ray;
use crate::hittable::HitRecord;

use crate::texture::Texture;
use crate::texture::SolidColor;

use std::rc::Rc;
use std::cell::RefCell;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct Lambertian {
    pub albedo: Rc<RefCell<dyn Texture>>,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray { origin: rec.p, direction: scatter_direction, tm: _r_in.time()};
        *attenuation = self.albedo.borrow().value(rec.u, rec.v, &rec.p);
        true
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Lambertian {albedo: Rc::new(RefCell::new(SolidColor::new(a.0, a.1, a.2)))}
    }
}

pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool{
        true
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray {origin: rec.p, direction: reflected + self.fuzz*random_in_unit_sphere(), tm: r_in.time()};
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0*Vec3::dot(v, n)*n
}

pub struct Dialectric {
    pub ir: f64
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front_face { 1.0 / self.ir} else { self.ir };
        
        let unit_direction: Vec3 = Vec3::unit_vector(r_in.direction());
        let cos_theta: f64 = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let mut direction: Vec3 = Vec3(0.0, 0.0, 0.0);

        if cannot_refract {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio)
        }

        *scattered = Ray { origin: rec.p, direction: direction, tm: r_in.time()};
        true
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

impl Dialectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
    }
}

pub struct DiffuseLight {
    emit: Rc<RefCell<dyn Texture>>
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.borrow().value(u, v, p)
    }
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        DiffuseLight {
            emit: Rc::new(RefCell::new(SolidColor { color_value: c }))
        }
    }
}

pub struct Isotropic {
    albedo: Rc<RefCell<Texture>>
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray {
            origin: rec.p,
            direction: random_in_unit_sphere(),
            tm: r_in.time(), 
        };
        *attenuation = self.albedo.borrow().value(rec.u, rec.v, &rec.p);
        return true;
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

impl Isotropic {
    pub fn new(c: Color) -> Self {
        Isotropic {
            albedo: Rc::new(RefCell::new(SolidColor::new(c[0], c[1], c[2])))
        }
    }
}