use ray_tracing_rust::rtweekend::random_double;
use std::f64::consts::PI;

fn main() {
    for _i in 0..200 {
        let r1 = random_double();
        let r2 = random_double();
        let x = (2.0*PI*r1).cos()*2.0*(r2*(1.0*r2).sqrt());
        let y = (2.0*PI*r1).sin()*2.0*(r2*(1.0*r2).sqrt());
        let z: f64 = 1.0 - 2.0*r2;
        println!("{} {} {}", x, y, z);
    }
}