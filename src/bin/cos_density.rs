use std::f64::consts::PI;
use ray_tracing_rust::vec3::{Vec3, random_cosine_direction};

fn f(d: &Vec3) -> f64 {
    let cos_theta: f64 = d.z();
    cos_theta*cos_theta*cos_theta
}

fn pdf(d: &Vec3) -> f64 {
    d.z() / PI
}

fn main() {
    let n = 1000000;

    let mut sum: f64 = 0.0;
    for _i in 0..n {
        let d: Vec3 = random_cosine_direction();
        sum += f(&d)/pdf(&d);
    }

    println!("PI/2 = {}", PI/2.0);
    println!("Estimate = {}", sum/n as f64);
}