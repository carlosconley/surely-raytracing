mod utils;

use crate::utils::random_double;

fn f(d: f64) -> f64 {
    8. * d.powf(1./3.)
}

fn pdf(x: f64) -> f64 {
    (3./8.) * x * x
}

fn main() {
    let n = 1;
    let mut sum = 0.;

    for _ in 0..n {
        let x = f(random_double());
        sum += x * x / pdf(x);
    }

    println!("{:.12}", sum / n as f64);
}
