use crate::vec3::Vec3;
use crate::vec3::Color;
use crate::vec3::random_unit_vector;

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
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray {origin: rec.p, direction: reflected};
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0*Vec3::dot(v, n)*n
}