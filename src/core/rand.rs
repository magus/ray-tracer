pub fn random_f64() -> f64 {
    // [0,1)
    rand::random_range(0.0..1.0)
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    // [min,max)
    rand::random_range(min..max)
}
