use ray_tracing_rust::rtweekend::random_double_range;
use ray_tracing_rust::rtweekend::random_double;

fn main() {
    let mut inside_circle: u64 = 0;
    let mut inside_circle_stratified: u64 = 0;
    let sqrt_n: u64 = 1000;

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let mut x: f64 = random_double_range(-1.0, 1.0);
            let mut y: f64 = random_double_range(-1.0, 1.0); 
            if x*x + y*y < 1.0 {
                inside_circle += 1;
            }
            x = 2.0*((i as f64 + random_double()) / sqrt_n as f64) - 1.0;
            y = 2.0*((j as f64 + random_double()) / sqrt_n as f64) - 1.0;
            if x*x + y*y < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }
    let n = sqrt_n as f64 * sqrt_n as f64;
    println!("Regular    Estimate of Pi = {:.10}", (4.0*(inside_circle as f64)) / n as f64);
    println!("Stratified Estimate of Pi = {:.10}", (4.0*(inside_circle_stratified as f64)) / n as f64);
}