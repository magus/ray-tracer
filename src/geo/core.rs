use crate::geo::Vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn random_unit_normal_direction(normal: &Vec3) -> Vec3 {
    let unit = random_unit();

    // in same general direction as normal (e.g. for a sphere, same hemisphere)
    if unit.dot(&normal) > 0.0 {
        unit
    } else {
        // otherwise, flip it so it is
        -unit
    }
}

pub fn random_unit() -> Vec3 {
    // rejection sample vector until it falls inside the unit
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.length_squared();
        if lensq <= 1.0 {
            let sqrtlensq = lensq.sqrt();
            // avoid potential division by zero for small values
            // e.g. 1e-160
            if sqrtlensq > 0.0 {
                return p / sqrtlensq;
            }
        }
    }
}
