mod vec3;
use vec3::Vec3;
use vec3::Point3;
use vec3::Color;
use vec3::random_double_range;

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
use material::Material;

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod moving_sphere;
use moving_sphere::MovingSphere;

mod rtweekend;
use rtweekend::random_double;

mod aabb;

mod texture;
use texture::CheckerTexture;

mod perlin;
use perlin::Perlin;

mod aarect;

mod abox;

mod constant_medium;
use constant_medium::ConstantMedium;

mod bvh_node;

use std::vec::Vec;

use std::rc::Rc;
use std::cell::RefCell;

// use rand::Rng;
use rand::distributions::Uniform;

use fastrand;

use std::time::Instant;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min };
    if x > max { return max };
    return x;
}

#[allow(dead_code)]
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

fn ray_color(r: &Ray, background: &Color, world: &mut dyn Hittable, depth: u64, mtr: &Rc<RefCell<DefaultMaterial>>) -> Color {
    // eprintln!("Current Depth {}", depth);
    let mut rec: HitRecord = HitRecord {
        p: Point3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        mat_ptr: mtr.clone(),
        t: 0.0,
        u: 0.0,
        v: 0.0,
        front_face: false,
    };

    // Limits the callback length. Guards against stack overflow
    if depth <= 0 {
        return Color(0.0, 0.0, 0.0);
    }

    let now_hit = Instant::now();
    if !world.hit(r, 0.001, f64::INFINITY.clone(), &mut rec) {
        // eprintln!("Depth {}, returned background", depth);
        return *background;    
    }
    // eprintln!("world hit: {}", now_hit.elapsed().as_nanos());

    let mut scattered: Ray = Ray {origin: Point3(0.0, 0.0, 0.0), direction: Vec3(0.0, 0.0, 0.0), tm: 0.0};
    let mut attenuation: Color = Color(0.0, 0.0, 0.0);
    let emitted = rec.mat_ptr.borrow().emitted(rec.u, rec.v, &rec.p);

    if !rec.mat_ptr.borrow().scatter(r, &rec, &mut attenuation, &mut scattered) {
        // eprintln!("Depth {}, returned emitted", depth);
        return emitted;
    }

    return emitted + attenuation * ray_color(&scattered, background, world, depth-1, mtr);
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

fn random_scene(zero_to_one: rand::distributions::Uniform<f64>) -> HittableList {
    let mut world: HittableList = HittableList { objects: Vec::new() };
    let zero_to_five_tenths_dist = Uniform::new(0.0f64, 0.5f64);
    // let five_tenths_to_one_dist = Uniform::new(0.5f64, 1.0f64);

    let checker = Rc::new(RefCell::new(CheckerTexture::new(Color(0.2, 0.3, 0.1),Color(0.9, 0.9, 0.9))));
    world.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, -1000.0, 0.0), radius: 1000.0, mat_ptr: Rc::new(RefCell::new(Lambertian{ albedo: checker })) })));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_double(zero_to_one);
            let center: Point3 = Point3(a as f64 + 0.9*random_double(zero_to_one), 0.2, b as f64 + 0.9*random_double(zero_to_one));

            if (center - Point3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<RefCell<dyn Material>>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = Color::random() * Color::random();
                    sphere_material = Rc::new(RefCell::new(Lambertian::new(&albedo)));
                    let center2 = center + Vec3(0.0, random_double(zero_to_five_tenths_dist), 0.0);
                    world.add(Rc::new(RefCell::new(MovingSphere { center0: center, center1: center2, time0: 0.0, time1: 1.0, radius: 0.2, mat_ptr: sphere_material })));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double(zero_to_five_tenths_dist);
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

fn two_spheres() -> HittableList {
    let mut objects: HittableList = HittableList {objects: Vec::new() };

    let checker1 = Rc::new(RefCell::new(CheckerTexture::new(Color(0.2, 0.3, 0.1),Color(0.9, 0.9, 0.9))));
    let checker2 = Rc::new(RefCell::new(CheckerTexture::new(Color(0.2, 0.3, 0.1),Color(0.9, 0.9, 0.9))));

    objects.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, -10.0, 0.0), radius: 10.0, mat_ptr: Rc::new(RefCell::new(Lambertian{ albedo: checker1 })) })));
    objects.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, 10.0, 0.0), radius: 10.0, mat_ptr: Rc::new(RefCell::new(Lambertian{ albedo: checker2 })) })));

    return objects;
}

fn two_perlin_spheres() -> HittableList {
    let mut objects: HittableList = HittableList {objects: Vec::new() };

    let pertext1 = Rc::new(RefCell::new(texture::NoiseTexture { noise: Perlin::new(), scale: 4.0}));
    let pertext2 = Rc::new(RefCell::new(texture::NoiseTexture { noise: Perlin::new(), scale: 4.0}));
    objects.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, -1000.0, 0.0), radius: 1000.0, mat_ptr: Rc::new(RefCell::new(Lambertian{ albedo: pertext1 })) })));
    objects.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, 2.0, 0.0), radius: 2.0, mat_ptr: Rc::new(RefCell::new(Lambertian{ albedo: pertext2 })) })));

    return objects;
}

fn earth() -> HittableList {
    let earth_texture = Rc::new(RefCell::new(texture::ImageTexture::new(String::from("pluto.jpg"))));
    let earth_surface = Rc::new(RefCell::new(Lambertian{ albedo: earth_texture }));
    let globe = Rc::new(RefCell::new(Sphere { center: Point3(0.0, 0.0, 0.0), radius: 2.0, mat_ptr: earth_surface}));

    let mut objects = HittableList {objects: Vec::new() };
    objects.add(globe);
    return objects;
}

fn simple_light() -> HittableList {
    let mut objects: HittableList = HittableList {objects: Vec::new() };

    let pertext1 = Rc::new(RefCell::new(texture::NoiseTexture { noise: Perlin::new(), scale: 4.0}));
    let pertext2 = Rc::new(RefCell::new(texture::NoiseTexture { noise: Perlin::new(), scale: 4.0}));
    objects.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, -1000.0, 0.0), radius: 1000.0, mat_ptr: Rc::new(RefCell::new(Lambertian{ albedo: pertext1 })) })));
    objects.add(Rc::new(RefCell::new(Sphere { center: Point3(0.0, 2.0, 0.0), radius: 2.0, mat_ptr: Rc::new(RefCell::new(Lambertian{ albedo: pertext2 })) })));

    let difflight =  Rc::new(RefCell::new(material::DiffuseLight::new(Color(4.0, 4.0, 4.0))));
    objects.add(Rc::new(RefCell::new(aarect::XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight))));

    return objects;
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

fn cornell_smoke() -> HittableList {
    let mut objects: HittableList = HittableList {objects: Vec::new() };

    let red   = Rc::new(RefCell::new(Lambertian::new( &Color(0.65, 0.05, 0.05))));
    let white = Rc::new(RefCell::new(Lambertian::new( &Color(0.73, 0.73, 0.73))));
    let green = Rc::new(RefCell::new(Lambertian::new( &Color(0.12, 0.45, 0.15))));
    let light = Rc::new(RefCell::new(material::DiffuseLight::new( Color(7.0, 7.0, 7.0))));

    objects.add(Rc::new(RefCell::new( aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green))));
    objects.add(Rc::new(RefCell::new( aarect::YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red))));
    objects.add(Rc::new(RefCell::new( aarect::XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light))));
    objects.add(Rc::new(RefCell::new( aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()))));
    objects.add(Rc::new(RefCell::new( aarect::XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()))));
    objects.add(Rc::new(RefCell::new( aarect::XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()))));
    
    let mut box1: Rc<RefCell<dyn Hittable>> = Rc::new(RefCell::new(abox::ABox::new(&Point3(0.0, 0.0, 0.0), &Point3(165.0, 330.0, 165.0), white.clone())));
    box1 = Rc::new(RefCell::new(hittable::RotateY::new(box1, 15.0)));
    box1 = Rc::new(RefCell::new(hittable::Translate::new(box1, Vec3(265.0, 0.0, 295.0))));

    let mut box2: Rc<RefCell<dyn Hittable>> = Rc::new(RefCell::new(abox::ABox::new(&Point3(0.0, 0.0, 0.0), &Point3(165.0, 165.0, 165.0), white.clone())));
    box2 = Rc::new(RefCell::new(hittable::RotateY::new(box2, -18.0)));
    box2 = Rc::new(RefCell::new(hittable::Translate::new(box2, Vec3(130.0, 0.0, 65.0))));

    objects.add(Rc::new(RefCell::new(ConstantMedium::new(box1, 0.01, Color(0.0, 0.0, 0.0) ))));
    objects.add(Rc::new(RefCell::new(ConstantMedium::new(box2, 0.01, Color(1.0, 1.0, 1.0) ))));

    return objects;
}

fn final_scene() -> HittableList {
    let mut boxes1: HittableList = HittableList {objects: Vec::new() };
    let ground = Rc::new(RefCell::new(Lambertian::new(&Color(1.20, 0.79, 0.64))));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64*w);
            let z0 = -1000.0 + (j as f64*w);
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Rc::new(RefCell::new(abox::ABox::new(&Point3(x0,y0,z0), &Point3(x1,y1,z1), ground.clone()))));
        }
    }

    let mut objects: HittableList = HittableList {objects: Vec::new() };
    objects.add(Rc::new(RefCell::new(bvh_node::BvhNode::new(&boxes1.objects, 0, boxes1.objects.len(), 0.0, 1.0))));

    let light = Rc::new(RefCell::new(material::DiffuseLight::new( Color(7.0, 7.0, 7.0))));
    objects.add(Rc::new(RefCell::new(aarect::XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone()))));

    let center1 = Point3(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(RefCell::new(Lambertian::new( &Color(1.14, 0.64, 1.20))));
    objects.add(Rc::new(RefCell::new( MovingSphere { center0: center1,
        center1: center2,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: moving_sphere_material,
    })));
    objects.add(Rc::new(RefCell::new(
        Sphere {
            center: Point3(260.0, 150.0, 45.0),
            radius: 50.0,
            mat_ptr: Rc::new(RefCell::new(Dialectric {ir: 1.5})),
        }
    )));

    objects.add(Rc::new(RefCell::new(
        Sphere {
            center: Point3(0.0, 150.0, 145.0), 
            radius: 50.0, 
            mat_ptr: Rc::new(RefCell::new(Metal { albedo: Color(0.8, 0.8, 0.9), fuzz: 1.0 }))
        }
    )));

    let mut boundary = Rc::new(RefCell::new(
        Sphere { 
            center: Point3(360.0, 150.0, 145.0), 
            radius: 70.0, 
            mat_ptr: Rc::new(RefCell::new(Dialectric {ir: 1.5 }))
    }));
    objects.add(boundary.clone());
    objects.add(Rc::new(RefCell::new(ConstantMedium::new(boundary.clone(), 0.2, Color(0.2, 0.4, 0.09)))));
    boundary = Rc::new(RefCell::new(
        Sphere {
            center: Point3(0.0, 0.0, 0.0), 
            radius: 5000.0, 
            mat_ptr: Rc::new(RefCell::new(Dialectric {ir: 1.5}))
        } 
    ));
    objects.add(Rc::new(RefCell::new(ConstantMedium::new(boundary.clone(), 0.0001, Color(1.0, 1.0, 1.0)))));

    let emat = Rc::new(RefCell::new(texture::ImageTexture::new(String::from("pluto.jpg"))));
    objects.add(Rc::new(RefCell::new(
        Sphere {
            center: Point3(400.0, 200.0, 400.0), 
            radius: 100.0, 
            mat_ptr: Rc::new(RefCell::new(Lambertian { albedo: emat }))
        }
    )));
    let pertext = Rc::new(RefCell::new(texture::NoiseTexture { noise: Perlin::new(), scale: 0.1}));
    objects.add(Rc::new(RefCell::new(
        Sphere {
            center: Point3(220.0, 280.0, 300.0), 
            radius: 80.0, 
            mat_ptr: Rc::new(RefCell::new(Lambertian { albedo: pertext }))
        }
    )));

    let mut boxes2: HittableList = HittableList {objects: Vec::new() };
    let white = Rc::new(RefCell::new(Lambertian::new( &Color(0.73, 0.73, 0.73))));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Rc::new(RefCell::new(
            Sphere {
                center: Point3::random_range(0.0, 165.0), 
                radius: 10.0, 
                mat_ptr:  white.clone()
            }
        )));
    }

    objects.add(Rc::new(RefCell::new(
        hittable::Translate::new(Rc::new(RefCell::new(
            hittable::RotateY::new(Rc::new(RefCell::new(
                bvh_node::BvhNode::new(&boxes2.objects, 0, boxes2.objects.len(), 0.0, 1.0))
            ), 15.0))), 
        Vec3(-100.0, 270.0, 395.0))
    )));

    return objects;
}

fn main() {
        // RNG
        let zero_to_one = Uniform::new(0.0f64, 1.0f64);
        let mut rng  = rand::thread_rng();

        // Image
        let mut aspect_ratio: f64 = 16.0/9.0;
        let mut image_width: u64 = 400;
        let mut samples_per_pixel: u64 = 100;
        const MAX_DEPTH: u64 = 50;
    
        // World
        // let mut world = random_scene(zero_to_one);
        let mut world: HittableList;

        let lookfrom: Point3;
        let lookat: Point3;
        let vfov: f64;
        let mut aperture: f64 = 0.0;
        let mut background: Color = Color(0.0, 0.0, 0.0);

        let case: u32 = 6;

        match case {
            1 => {
                world = random_scene(zero_to_one);
                background = Color(0.70, 0.80, 1.00);
                lookfrom = Point3(13.0, 2.0, 3.0);
                lookat = Point3(0.0, 0.0, 0.0);
                vfov = 20.0;
                aperture = 0.1;
            },
            2 => {
                world = two_spheres();
                background = Color(0.70, 0.80, 1.00);
                lookfrom = Point3(13.0, 2.0, 3.0);
                lookat = Point3(0.0, 0.0, 0.0);
                vfov = 20.0;
            },
            3 => {
                world = two_perlin_spheres();
                background = Color(0.70, 0.80, 1.00);
                lookfrom = Point3(13.0, 2.0, 3.0);
                lookat = Point3(0.0, 0.0, 0.0);
                vfov= 20.0;
            },
            4 => {
                world = earth();
                background = Color(0.70, 0.80, 1.00);
                lookfrom = Point3(13.0, 2.0, 3.0);
                lookat = Point3(0.0, 0.0, 0.0);
                vfov = 20.0;
            },
            5 => {
                world = simple_light();
                samples_per_pixel = 400;
                background = Color(0.0, 0.0, 0.0);
                lookfrom = Point3(26.0, 3.0, 6.0);
                lookat = Point3(0.0, 2.0, 0.0);
                vfov = 20.0;
            },
            6 => {
                world = cornell_box();
                aspect_ratio = 1.0;
                image_width = 600;
                samples_per_pixel = 200;
                background = Color(0.0, 0.0, 0.0);
                lookfrom = Point3(278.0, 278.0, -800.0);
                lookat = Point3(278.0, 278.0, 0.0);
                vfov = 40.0;
            }
            7 => {
                world = cornell_smoke();
                aspect_ratio = 1.0;
                image_width = 600;
                samples_per_pixel = 200;
                lookfrom = Point3(278.0, 278.0, -800.0);
                lookat = Point3(278.0, 278.0, 0.0);
                vfov = 40.0;
            } 
            8 => {
                world = final_scene();
                aspect_ratio = 1.0;
                image_width = 100;
                samples_per_pixel = 1000;
                background = Color(0.0, 0.0, 0.0);
                lookfrom = Point3(478.0, 278.0, -600.0);
                lookat = Point3(278.0, 278.0, 0.0);
                vfov = 40.0;
            },
            _ => {
                world = final_scene();
                aspect_ratio = 1.0;
                image_width = 800;
                samples_per_pixel = 10000;
                background = Color(0.0, 0.0, 0.0);
                lookfrom = Point3(478.0, 278.0, -600.0);
                lookat = Point3(278.0, 278.0, 0.0);
                vfov = 40.0;
            }
        }

        let image_height: u64 = (image_width as f64/aspect_ratio) as u64;

        // Camera
        // let lookfrom: Point3 = Point3(13.0, 2.0, 3.0);
        // let lookat: Point3 = Point3(0.0, 0.0, 0.0);
        // let vup: Vec3 = Vec3(0.0, 1.0, 0.0);
        // let dist_to_focus: f64 = 10.0;
        // let aperture: f64 = 0.1;
        let vup = Vec3(0.0, 1.0, 0.0);
        let dist_to_focus: f64 = 10.0;

        // let cam: Camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0); 
        let cam: Camera = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0); 
        let mtr = Rc::new(RefCell::new(DefaultMaterial));
        // Render
        print!("P3\n{} {}\n255\n", image_width, image_height);
        for j in (0..image_height).rev() {
            eprintln!("Scanlines remaining: {}", j);
            for i in 0..image_width {
                let now = Instant::now();
                let mut pixel_color: Color = Color(0.0, 0.0, 0.0);
                let mut rand_t: u128 = 0;
                let mut rand_ray: u128 = 0;
                let mut rand_pixel: u128 = 0;
                for _k in 0..samples_per_pixel {
                    let now_r = Instant::now();
                    let u: f64 = (i as f64 + fastrand::f64()) / (image_width - 1) as f64;
                    let v: f64 = (j as f64 + fastrand::f64()) / (image_height - 1) as f64;
                    rand_t += now_r.elapsed().as_nanos();
                    let now_ray = Instant::now();
                    let r: Ray = cam.get_ray(u, v);
                    rand_ray += now_ray.elapsed().as_nanos();
                    let now_pixel = Instant::now();
                    pixel_color += ray_color(&r, &background, &mut world, MAX_DEPTH, &mtr);
                    rand_pixel += now_pixel.elapsed().as_nanos();
                }
                write_color(pixel_color, samples_per_pixel);
                eprintln!("Line:{}, Pixel:{}, Time elapsed: {}", j, i, now.elapsed().as_nanos());
                eprintln!("Line:{}, Pixel:{}, Time elapsed for rand: {}", j, i, rand_t / samples_per_pixel as u128);
                eprintln!("Line:{}, Pixel:{}, Time elapsed for ray: {}", j, i, rand_ray / samples_per_pixel as u128);
                eprintln!("Line:{}, Pixel:{}, Time elapsed for pixel color: {}", j, i, rand_pixel / samples_per_pixel as u128);

            }
        }
    
    eprintln!("Done. ");
}


