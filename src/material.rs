use crate::pdf::CosinePdf;
use crate::vec3::Vec3;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::refract;

use crate::pdf::Pdf;

use crate::ray::Ray;
use crate::hittable::HitRecord;

use crate::texture::Texture;
use crate::texture::SolidColor;

use crate::rtweekend::random_double;

use crate::pdf::SpherePdf;

use std::f64::consts::PI;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Rc<RefCell<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    pub albedo: Rc<RefCell<dyn Texture>>,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo.borrow().value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Rc::new(RefCell::new(CosinePdf::new(&rec.normal)));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        let cos_theta = Vec3::dot(_rec.normal, Vec3::unit_vector(_scattered.direction()));
        if cos_theta < 0.0 { 0.0 } else { cos_theta/PI }
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Lambertian {albedo: Rc::new(RefCell::new(SolidColor::new(a.0, a.1, a.2)))}
    }
}

pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool{
        true
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord,  _u: f64, _v: f64, _p: &Point3) -> Color {
        Color(0.0, 0.0, 0.0)
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo;
        // srec.pdf_ptr = std::ptr::null::<Rc<RefCell<dyn Pdf>>>();
        srec.skip_pdf = true;
        let reflected: Vec3 = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        srec.skip_pdf_ray = Ray { origin: rec.p, direction: reflected + self.fuzz*random_in_unit_sphere(), tm: r_in.time()};
        true
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0*Vec3::dot(*v, *n)**n
}

pub struct Dialectric {
    pub ir: f64
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color(1.0, 1.0, 1.0);
        // srec.pdf_ptr
        srec.skip_pdf = true;
        let refraction_ratio: f64 = if rec.front_face { 1.0 / self.ir} else { self.ir };
        
        let unit_direction: Vec3 = Vec3::unit_vector(r_in.direction());
        let cos_theta: f64 = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Dialectric::reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio)
        }

        srec.skip_pdf_ray = Ray { origin: rec.p, direction: direction, tm: r_in.time()};
        true
    }
}

impl Dialectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0.powi(2);
        r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
    }
}

pub struct DiffuseLight {
    emit: Rc<RefCell<dyn Texture>>
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            return Color(0.0, 0.0, 0.0);
        }
        return self.emit.borrow().value(u, v, p);
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
    albedo: Rc<RefCell<dyn Texture>>,
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo.borrow().value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Rc::new(RefCell::new(SpherePdf()));
        srec.skip_pdf = false;
        return true;
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        1.0 * (4.0 * PI)
    }
}

impl Isotropic {
    pub fn new(c: Color) -> Self {
        Isotropic {
            albedo: Rc::new(RefCell::new(SolidColor::new(c[0], c[1], c[2])))
        }
    }

    pub fn new_texture(a: Rc<RefCell<dyn Texture>>) -> Self {
        Isotropic { albedo: a }
    }
}