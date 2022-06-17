use ray_tracing_rust::rtweekend::random_double_range;

fn pdf(x: f64) -> f64{
    (3.0*x*x)/8.0
}

fn main() {
    let n: u64 = 1;
    let mut sum = 0.0;
    for _i in 0..n {
        let x: f64 = random_double_range(0.0, 8.0).powf(1.0/3.0);
        sum += (x*x)/pdf(x);
    }
    println!("I = {}", (sum/n as f64));
}