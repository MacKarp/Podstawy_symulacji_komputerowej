use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let lambda = rng.gen_range(0.0..100.0);
    let result = poisson_generator(lambda);
    println!("Result: {}", result);
}

fn poisson_generator(lambda: f64) -> i64 {
    let exp_lambda = (-lambda).exp();
    let mut result = -1;
    let mut producted_unit: f64 = 1.0;
    let mut random_unit: f64;

    while producted_unit > exp_lambda {
        let mut rng = thread_rng();
        random_unit = rng.gen();
        producted_unit *= random_unit;
        result += 1;
    }
    result
}