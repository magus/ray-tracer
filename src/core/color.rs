use crate::geo::Interval;
use crate::geo::Vec3;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(Vec3);

impl Color {
    pub fn new(rf: f64, gf: f64, bf: f64) -> Self {
        Color(Vec3::new(rf, gf, bf))
    }
}

impl std::ops::Deref for Color {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const INTENSITY: Interval = Interval::new(0.0, 0.9999);

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // transform for gamma 2
        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        // translate [0,1] to rgb byte range [0,255]
        let r = (256.0 * INTENSITY.clamp(r)) as u32;
        let g = (256.0 * INTENSITY.clamp(g)) as u32;
        let b = (256.0 * INTENSITY.clamp(b)) as u32;

        write!(f, "{} {} {}", r, g, b)
    }
}

impl From<Color> for Vec3 {
    fn from(c: Color) -> Self {
        *c
    }
}

impl From<&Color> for Vec3 {
    fn from(c: &Color) -> Self {
        **c
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let a = Color::new(0.0, 1.0, 0.5);
        assert_eq!(format!("{a}"), "0 255 181");
    }

    #[test]
    fn test_from_color() {
        let a = Color::new(0.0, 0.0, 0.0);
        let b = Vec3::from(a);
        assert_eq!(b, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_from_vec3() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Color::from(a);
        assert_eq!(b, Color::new(0.0, 0.0, 0.0));
    }
}
