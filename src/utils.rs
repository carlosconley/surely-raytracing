use rand::Rng;

pub const INF: f64 = f64::INFINITY;

pub fn random_double() -> f64 {
    rand::random::<f64>()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_int(min: i64, max: i64) -> i64 {
    rand::thread_rng().gen_range(min..=max)
}
