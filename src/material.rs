use crate::vec3::Vec3;
use crate::vec3::Color;
use crate::vec3::random_unit_vector;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::refract;

use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray { origin: rec.p, direction: scatter_direction};
        *attenuation = self.albedo;
        true
    }
}

pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool{
        true
    }
}



pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray {origin: rec.p, direction: reflected + self.fuzz*random_in_unit_sphere()};
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.0
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

        *scattered = Ray { origin: rec.p, direction: direction};
        true
    }
}

impl Dialectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
    }
}