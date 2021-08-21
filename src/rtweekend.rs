use rand::Rng;

pub fn random_double(distribution: rand::distributions::Uniform<f64>) -> f64 {
    let mut rng  = rand::thread_rng();
    rng.sample(distribution)
}