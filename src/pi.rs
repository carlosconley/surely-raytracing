mod utils;

use crate::utils::{random_range, random_double};

fn main() {
    let mut inside_circle = 0;
    let mut inside_circle_stratified = 0;
    let sqrt_n = 1000;

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = random_range(-1., 1.);
            let y = random_range(-1., 1.);

            if x * x + y * y < 1. {
                inside_circle += 1;
            }

            let x = 2.*((i as f64 + random_double()) / sqrt_n as f64) - 1.;
            let y = 2.*((j as f64 + random_double()) / sqrt_n as f64) - 1.;

            if x*x + y * y < 1. {
                inside_circle_stratified += 1;
            }
        }
    }

    println!("Regular    Estimate of Pi = {:.12}", (4. * inside_circle as f64) / (sqrt_n * sqrt_n) as f64);
    println!("Stratified Estimate of Pi = {:.12}", (4. * inside_circle_stratified as f64) / (sqrt_n * sqrt_n) as f64);
}
