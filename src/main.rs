use std::ops::Neg;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use std::fmt;

#[derive(Copy, Clone)]
struct Vec3(f64, f64, f64);

use Vec3 as Point3;
use Vec3 as Color;

impl Vec3 {

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn length(&self) -> f64 {
        self.length_square().sqrt()
    }

    fn length_square(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    fn dot(u: Vec3, v: Vec3)-> f64 {
        u.0*v.0 + u.1*v.1 + u.2*v.2
    }

    fn cross(u: Vec3, v: Vec3)-> Vec3 {
        Vec3(u.1*v.2 - u.2*v.1, u.2*v.0 - u.0*v.2, u.0*v.1 - u.1*v.0)
    }

    fn unit_vector(v: Vec3) -> Vec3 {
        let k = v.length();
        v / k
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg (self) -> Vec3{
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Index<u32> for Vec3 {
    type Output = f64;

    fn index(&self, i: u32) -> &f64 {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => &self.2,
        }
    }
}

impl IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, i: u32) -> &mut f64{
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => &mut self.2,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }    
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.0 /= t;
        self.1 /= t;
        self.2 /= t;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0*other.0, self.1*other.1, self.2*other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3(t*self.0, t*self.1, t*self.2)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, other: Vec3) -> Vec3 {
        other*self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        (1.0/t) * self
    }
}

struct Ray {
    origin: Point3,
    direction: Vec3 
}

impl Ray {
    fn at(self, t: f64) -> Point3 {
        self.origin + t*self.direction
    }

    fn origin(self) -> Point3 {
        self.origin
    }

    fn direction(self) -> Vec3 {
        self.direction
    }
}

fn first_image() {
    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGTH: u32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGTH);

    for j in (0..256).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..256 {
            let r: f32 = i as f32/(IMAGE_WIDTH-1) as f32;
            let g: f32 = j as f32/(IMAGE_HEIGTH-1) as f32;
            let b: f32 = 0.25;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib)
        }
    }

    eprintln!("Done. ");
}

fn second_image() {
    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGTH: u32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGTH);

    for j in (0..256).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..256 {
            let pixel_color: Color = Color(i as f64/(IMAGE_WIDTH-1) as f64, j as f64/(IMAGE_HEIGTH-1) as f64, 0.25 as f64);
            write_color(pixel_color);
        }
    }

    eprintln!("Done. ");
}

fn third_image() {
    // Image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGTH: u64 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u64;

    // Camera
    let viewport_height: f64= 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point3(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGTH);

    for j in (0..IMAGE_HEIGTH).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let r: Ray = Ray {origin: origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};
            let pixel_color: Color = ray_color(r);
            write_color(pixel_color);
        }
    }

    eprintln!("Done. ");
}

fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Color(1.0, 1.0, 1.0) + t*Color(0.5, 0.7, 1.0)
}

fn write_color(pixel_color: Color) {
    println!("{} {} {}", 
        (255.999 * pixel_color.x()) as u64, 
        (255.999 * pixel_color.y()) as u64,
        (255.999 * pixel_color.z()) as u64)
}

fn main() {
    // let a = Vec3(1.0, 2.0, 3.0);
    // let b = Vec3(2.0, 3.0, 4.0);
    // let c : Vec3 = 2.0*a;
    // println!("This is my vector: {}", c);
    third_image();
}


