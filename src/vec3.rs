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
use rand::Rng;
use rand::distributions::Uniform;

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub use Vec3 as Point3;
pub use Vec3 as Color;

impl Vec3 {

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }

    pub fn length_square(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn dot(u: Vec3, v: Vec3)-> f64 {
        u.0*v.0 + u.1*v.1 + u.2*v.2
    }

    pub fn cross(u: Vec3, v: Vec3)-> Vec3 {
        Vec3(u.1*v.2 - u.2*v.1, u.2*v.0 - u.0*v.2, u.0*v.1 - u.1*v.0)
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        let k = v.length();
        v / k
    }

    pub fn random() -> Vec3 {
        Vec3(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        const S: f64 = 1e-8;
        (self.0.abs() < S) && (self.1.abs() < S) && (self.2.abs() < S)
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

fn random_double() -> f64 {
    let a: f64 = rand::thread_rng().gen_range(0.0..1.0);
    a
}

fn random_double_range(min: f64, max: f64) -> f64 {
    let a: f64 = rand::thread_rng().gen_range(0.0..1.0);
    min + (max - min)*a
}

// Initially the plan was to rely on methods in Vec3 but rand is slow. Sampling from a uniform is faster
pub fn random_in_unit_sphere() -> Vec3 {
    let random_space = Uniform::new(-1.0f64, 1.0f64);
    let mut rng  = rand::thread_rng();
    let mut p: Vec3 = Vec3(rng.sample(random_space), rng.sample(random_space), rng.sample(random_space));
    loop {
        if p.length_square() >= 1.0 {
            p =  Vec3(rng.sample(random_space), rng.sample(random_space), rng.sample(random_space));
            continue
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3{
    let in_unit_sphere = random_in_unit_sphere();
    if Vec3::dot(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}