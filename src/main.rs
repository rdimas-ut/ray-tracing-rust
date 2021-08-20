mod vec3;
use vec3::Vec3;
use vec3::Point3;
use vec3::Color;
use vec3::PI;

mod camera;
use camera::Camera;

mod ray;
use ray::Ray;

mod hittable;
use hittable::HitRecord;
use hittable::Hittable;

mod material;
use material::DefaultMaterial;
use material::Lambertian;
use material::Metal;
use material::Dialectric;

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

use std::vec::Vec;

use std::rc::Rc;
use std::cell::RefCell;

use rand::Rng;
use rand::distributions::Uniform;

use std::default::Default;
use std::f64::MAX;

// Constants
const INFINITY: f64 = MAX;

// Utility Functions
fn random_double() -> f64 {
    let a: f64 = rand::thread_rng().gen_range(0.0..1.0);
    a
}

fn random_double_range(min: f64, max: f64) -> f64 {
    let a: f64 = rand::thread_rng().gen_range(0.0..1.0);
    min + (max - min)*a
}


fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min };
    if x > max { return max };
    return x;
}


fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let a: f64 = r.direction().length_square();
    let half_b: f64 = Vec3::dot(oc, r.direction());
    let c: f64 = oc.length_square() - radius*radius;
    let discriminant: f64 = (half_b*half_b) - (a*c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray, world: &mut dyn Hittable, depth: u64) -> Color {
    let mut rec: HitRecord = HitRecord {
        p: Point3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        mat_ptr: Rc::new(RefCell::new(DefaultMaterial)),
        t: 0.0,
        front_face: false,
    };

    // Limits the callback length. Guards against stack overflow
    if depth <= 0 {
        return Color(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray {origin: Point3(0.0, 0.0, 0.0), direction: Vec3(0.0, 0.0, 0.0)};
        let mut attenuation: Color = Color(0.0, 0.0, 0.0);
        if rec.mat_ptr.borrow().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Color(0.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Color(1.0, 1.0, 1.0) + t*Color(0.5, 0.7, 1.0)
}

fn write_color(pixel_color: Color, samples_per_pixel: u64) {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    // Divide the color by the number of samples
    let scale: f64 = 1.0/samples_per_pixel as f64;

    r *= scale; r = r.sqrt();
    g *= scale; g = g.sqrt();
    b *= scale; b = b.sqrt();

    println!("{} {} {}", 
        (256.0 * clamp(r, 0.0, 0.999)) as u64, 
        (256.0 * clamp(g, 0.0, 0.999)) as u64,
        (256.0 * clamp(b, 0.0, 0.999)) as u64)
}

fn main() {
        // Image
        const ASPECT_RATIO: f64 = 16.0/9.0;
        const IMAGE_WIDTH: u64 = 400;
        const IMAGE_HEIGTH: u64 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u64;
        const SAMPLES_PER_PIXEL: u64 = 100;
        const MAX_DEPTH: u64 = 50;
    
        // World
        let R: f64 = (PI/4.0).cos();
        let mut world: HittableList = HittableList { objects: Vec::new() };
    
        let material_ground = Rc::new(RefCell::new(Lambertian{ albedo: Color(0.8, 0.8, 0.0) }));
        let material_center = Rc::new(RefCell::new(Lambertian{ albedo: Color(0.1, 0.2, 0.5) }));
        let material_left   = Rc::new(RefCell::new(Dialectric {ir: 1.5 }));
        let material_left2   = Rc::new(RefCell::new(Dialectric {ir: 1.5 }));
        let material_right  = Rc::new(RefCell::new(Metal {albedo: Color(0.8, 0.6, 0.2), fuzz: 0.0 }));
    
        world.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, -100.5, -1.0), radius: 100.0, mat_ptr: material_ground })));
        world.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, 0.0, -1.0), radius: 0.5, mat_ptr: material_center })));
        world.add(Rc::new(RefCell::new(Sphere { center: Point3(-1.0, 0.0, -1.0), radius: 0.5, mat_ptr: material_left })));
        world.add(Rc::new(RefCell::new(Sphere { center: Point3(-1.0, 0.0, -1.0), radius: -0.4, mat_ptr: material_left2 })));
        world.add(Rc::new(RefCell::new(Sphere { center: Point3(1.0, 0.0, -1.0), radius: 0.5, mat_ptr: material_right })));
    
        // Camera
        let cam: Camera = Camera::new(Point3(-2.0,2.0,1.0), Point3(0.0,0.0,-1.0), Vec3(0.0,1.0,0.0), 90.0, ASPECT_RATIO); 
    
        // RNG
        let random_space = Uniform::new(0.0f64, 1.0f64);
        let mut rng  = rand::thread_rng();
    
        // Render
        print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGTH);
    
        for j in (0..IMAGE_HEIGTH).rev() {
            eprintln!("Scanlines remaining: {}", j);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color: Color = Color(0.0, 0.0, 0.0);
                for _k in 0..SAMPLES_PER_PIXEL {
                    let u: f64 = (i as f64 + rng.sample(random_space)) / (IMAGE_WIDTH - 1) as f64;
                    let v: f64 = (j as f64 + rng.sample(random_space)) / (IMAGE_HEIGTH - 1) as f64;
                    let r: Ray = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &mut world, MAX_DEPTH);
                }
                write_color(pixel_color, SAMPLES_PER_PIXEL);
            }
        }
    
    eprintln!("Done. ");
}


