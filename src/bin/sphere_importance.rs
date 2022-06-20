use ray_tracing_rust::vec3::{Vec3, random_unit_vector};
use std::f64::consts::PI;

fn pdf(_p: &Vec3) -> f64{
    1.0/(4.0*PI)
}

fn main() {
    let n: u64 = 1000000;
    let mut sum = 0.0;
    for _i in 0..n {
        let d: Vec3 = random_unit_vector();
        let cosine_squared: f64 = d.z()*d.z();
        sum += cosine_squared/pdf(&d);
    }
    println!("I = {}", (sum/n as f64));
}