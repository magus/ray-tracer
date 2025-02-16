pub fn float(a: f64, b: f64, digits: i32) {
    let epsilon = 0.1_f64.powi(digits);
    assert!(
        (a - b).abs() < epsilon,
        "assertion failed: got {}, expected {} (epsilon: {})",
        a,
        b,
        epsilon
    );
}
