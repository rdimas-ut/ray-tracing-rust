use std::f64::consts::PI;

use ray_tracing_rust::rtweekend::random_double;

fn f(_r1: f64, r2: f64) -> f64 {
    let z: f64 = 1.0 - r2;
    let cos_theta: f64 = z;
    cos_theta*cos_theta*cos_theta
}

fn pdf(_r1: f64, _r2: f64) -> f64 {
    1.0 * (2.0*PI)
}

fn main() {
    let n = 1000000;

    let mut sum: f64 = 0.0;
    for _i in 0..n {
        let r1 = random_double();
        let r2 = random_double();
        sum += f(r1, r2)/pdf(r1, r2);
    }

    println!("PI/2 = {}", PI/2.0);
    println!("Estimate = {}", sum/n as f64);
}