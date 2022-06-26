use ray_tracing_rust::color;
use ray_tracing_rust::rtweekend::random_double_range;
use ray_tracing_rust::vec3::Vec3;
use ray_tracing_rust::vec3::Point3;
use ray_tracing_rust::vec3::Color;

use ray_tracing_rust::color::write_color;

use ray_tracing_rust::camera::Camera;

use ray_tracing_rust::ray::Ray;

use ray_tracing_rust::hittable;
use ray_tracing_rust::hittable::HitRecord;
use ray_tracing_rust::hittable::Hittable;

use ray_tracing_rust::material;
use ray_tracing_rust::material::DefaultMaterial;
use ray_tracing_rust::material::Lambertian;

use ray_tracing_rust::hittable_list::HittableList;

use ray_tracing_rust::rtweekend::random_double;

use ray_tracing_rust::aarect;

use ray_tracing_rust::abox;

use ray_tracing_rust::pdf::Pdf;
use ray_tracing_rust::pdf::CosinePdf;

use std::f64::consts::PI;
use std::vec::Vec;

use std::rc::Rc;
use std::cell::RefCell;

fn ray_color(r: &Ray, background: &Color, world: &mut dyn Hittable, depth: u64) -> Color {
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

    if !world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        return *background;    
    }

    let mut scattered: Ray = Ray {origin: Point3(0.0, 0.0, 0.0), direction: Vec3(0.0, 0.0, 0.0), tm: 0.0};
    let mut attenuation: Color = Color(0.0, 0.0, 0.0);
    let mut pdf: f64 = 0.0;
    let color_from_emission = rec.mat_ptr.borrow().emitted(r, &rec, rec.u, rec.v, &rec.p);

    if !rec.mat_ptr.borrow().scatter(r, &rec, &mut attenuation, &mut scattered, &mut pdf) {
        return color_from_emission;
    }

    let surface_pdf: CosinePdf = CosinePdf::new(&rec.normal);
    scattered = Ray {origin: rec.p, direction: surface_pdf.generate(), tm: r.time()};
    let pdf_val = surface_pdf.value(&scattered.direction());

    let scattering_pdf: f64 = rec.mat_ptr.borrow().scattering_pdf(r, &rec, &mut scattered);

    let color_from_scatter = (attenuation * scattering_pdf * ray_color(&scattered, background, world, depth-1)) / pdf_val;

    return color_from_emission + color_from_scatter;
}

fn cornell_box() -> HittableList {
    let mut objects: HittableList = HittableList {objects: Vec::new() };

    let red   = Rc::new(RefCell::new(Lambertian::new( &Color(0.65, 0.05, 0.05))));
    let white = Rc::new(RefCell::new(Lambertian::new( &Color(0.73, 0.73, 0.73))));
    let green = Rc::new(RefCell::new(Lambertian::new( &Color(0.12, 0.45, 0.15))));
    let light = Rc::new(RefCell::new(material::DiffuseLight::new( Color(15.0, 15.0, 15.0))));

    objects.add(Rc::new(RefCell::new( aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green))));
    objects.add(Rc::new(RefCell::new( aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red))));
    objects.add(Rc::new(RefCell::new( aarect::XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light))));
    objects.add(Rc::new(RefCell::new( aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()))));
    objects.add(Rc::new(RefCell::new( aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()))));
    objects.add(Rc::new(RefCell::new( aarect::XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()))));
    
    let mut box1: Rc<RefCell<dyn Hittable>> = Rc::new(RefCell::new(abox::ABox::new(&Point3(0.0, 0.0, 0.0), &Point3(165.0, 330.0, 165.0), white.clone())));
    box1 = Rc::new(RefCell::new(hittable::RotateY::new(box1, 15.0)));
    box1 = Rc::new(RefCell::new(hittable::Translate::new(box1, Vec3(265.0, 0.0, 295.0))));
    objects.add(box1);

    let mut box2: Rc<RefCell<dyn Hittable>> = Rc::new(RefCell::new(abox::ABox::new(&Point3(0.0, 0.0, 0.0), &Point3(165.0, 165.0, 165.0), white.clone())));
    box2 = Rc::new(RefCell::new(hittable::RotateY::new(box2, -18.0)));
    box2 = Rc::new(RefCell::new(hittable::Translate::new(box2, Vec3(130.0, 0.0, 65.0))));
    objects.add(box2);

    return objects;
}

fn main() {

        // Image
        let aspect_ratio: f64 = 1.0/1.0;
        let image_width: u64 = 500;
        let image_height: u64 = (image_width as f64/aspect_ratio) as u64;
        let samples_per_pixel: u64 = 10;
        const MAX_DEPTH: u64 = 50;
    
        // World
        let mut world: HittableList = cornell_box();
        let background: Color = Color(0.0, 0.0, 0.0);

        // Camera
        let lookfrom: Point3 = Point3(278.0, 278.0, -800.0);
        let lookat: Point3 = Point3(278.0, 278.0, 0.0);
        let vup: Vec3 = Vec3(0.0, 1.0, 0.0);
        let dist_to_focus: f64 = 10.0;
        let aperture: f64 = 0.0;
        let vfov: f64 = 40.0;
        let time0: f64 = 0.0;
        let time1: f64 = 1.0;
        
        let cam: Camera = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, time0, time1); 
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
                    pixel_color += ray_color(&r, &background, &mut world, MAX_DEPTH);
                }
                write_color(pixel_color, samples_per_pixel);
            }
        }
    eprintln!("Done. ");
}