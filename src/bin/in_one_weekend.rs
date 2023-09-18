use ray_tracing_rust::vec3::Vec3;
use ray_tracing_rust::vec3::Point3;
use ray_tracing_rust::vec3::Color;

use ray_tracing_rust::color::write_color;

use ray_tracing_rust::camera_first_week::Camera;

use ray_tracing_rust::ray_first_week::Ray;

use ray_tracing_rust::hittable_first_week::HitRecord;
use ray_tracing_rust::hittable_first_week::Hittable;

use ray_tracing_rust::material_first_week::DefaultMaterial;
use ray_tracing_rust::material_first_week::Lambertian;
use ray_tracing_rust::material_first_week::Metal;
use ray_tracing_rust::material_first_week::Dialectric;
use ray_tracing_rust::material_first_week::Material;

use ray_tracing_rust::hittable_list_first_week::HittableList;

use ray_tracing_rust::sphere_first_week::Sphere;

use ray_tracing_rust::rtweekend::random_double;
use ray_tracing_rust::rtweekend::random_double_range;

use std::vec::Vec;

use std::rc::Rc;
use std::cell::RefCell;

fn ray_color(r: &Ray, world: &mut dyn Hittable, depth: u64) -> Color {
    let mut rec: HitRecord = HitRecord {
        p: Point3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        mat_ptr: Rc::new(RefCell::new(DefaultMaterial)),
        t: 0.0,
        u: 0.0,
        v: 0.0,
        front_face: false,
    };

    // Limits the callback length. Guards against stack overflow
    if depth <= 0 {
        return Color(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray {origin: Point3(0.0, 0.0, 0.0), direction: Vec3(0.0, 0.0, 0.0), tm: 0.0};
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

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList { objects: Vec::new() };

    let ground_material = Rc::new(RefCell::new(Lambertian::new(&Color(0.5, 0.5, 0.5))));
    world.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, -1000.0, 0.0), radius: 1000.0, mat_ptr: ground_material })));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_double();
            let center: Point3 = Point3(a as f64 + 0.9*random_double(), 0.2, b as f64 + 0.9*random_double());

            if (center - Point3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<RefCell<dyn Material>>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = Color::random() * Color::random();
                    sphere_material = Rc::new(RefCell::new(Lambertian::new(&albedo)));
                    world.add(Rc::new(RefCell::new(Sphere { center: center, radius: 0.2, mat_ptr: sphere_material })));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(RefCell::new(Metal{ albedo: albedo, fuzz: fuzz }));
                    world.add(Rc::new(RefCell::new(Sphere { center: center, radius: 0.2, mat_ptr: sphere_material })));
                } else {
                    // glass
                    sphere_material = Rc::new(RefCell::new(Dialectric{ ir: 1.5 }));
                    world.add(Rc::new(RefCell::new(Sphere { center: center, radius: 0.2, mat_ptr: sphere_material })));
                }
            }
        }
    }

    
    let material1 = Rc::new(RefCell::new(Dialectric{ ir: 1.5 }));
    world.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, 1.0, 0.0), radius: 1.0, mat_ptr: material1 })));

    let material2 = Rc::new(RefCell::new(Lambertian::new(&Color(0.4, 0.2, 0.1))));
    world.add(Rc::new(RefCell::new(Sphere { center: Point3(-4.0, 1.0, 0.0), radius: 1.0, mat_ptr: material2 })));

    let material3 = Rc::new(RefCell::new(Metal{ albedo: Color(0.7, 0.6, 0.5), fuzz: 0.0 }));
    world.add(Rc::new(RefCell::new(Sphere { center: Point3(4.0, 1.0, 0.0), radius: 1.0, mat_ptr: material3 })));

    return world;
}

fn main() {

        // Image
        let aspect_ratio: f64 = 16.0/9.0;
        let image_width: u64 = 1200;
        let image_height: u64 = (image_width as f64/aspect_ratio) as u64;
        let samples_per_pixel: u64 = 500;
        const MAX_DEPTH: u64 = 50;
    
        // World
        let mut world: HittableList = random_scene();

        // Camera
        let lookfrom: Point3 = Point3(13.0, 2.0, 3.0);
        let lookat: Point3 = Point3(0.0, 0.0, 0.0);
        let vup = Vec3(0.0, 1.0, 0.0);
        let dist_to_focus: f64 = 10.0;
        let aperture: f64 = 0.1;

        let cam: Camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 0.0); 
        // Render
        print!("P3\n{} {}\n255\n", image_width, image_height);
        for j in (0..image_height).rev() {
            eprintln!("Scanlines remaining: {}", j);
            for i in 0..image_width {
                let mut pixel_color: Color = Color(0.0, 0.0, 0.0);
                for _k in 0..samples_per_pixel {
                    let u: f64 = (i as f64 + random_double()) / (image_width - 1) as f64;
                    let v: f64 = (j as f64 + random_double()) / (image_height - 1) as f64;
                    let r: Ray = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &mut world, MAX_DEPTH);
                }
                write_color(pixel_color, samples_per_pixel);
            }
        }
    eprintln!("Done. ");
}