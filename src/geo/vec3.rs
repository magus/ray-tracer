use crate::core::{random_f64, random_f64_range};
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn validate(self) {
        assert!(
            self.x.is_finite() && self.y.is_finite() && self.z.is_finite(),
            "non-finite values not allowed"
        );
    }

    pub fn inew(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3::new(x as f64, y as f64, z as f64)
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
        )
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Eq for Vec3 {}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vec3::new(x, y, z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Vec3::new(x, y, z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vec3::new(x, y, z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl ops::Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl ops::IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Vec3::new(x, y, z)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let x = rhs.x * self;
        let y = rhs.y * self;
        let z = rhs.z * self;
        Vec3::new(x, y, z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vec3::new(x, y, z)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Vec3::new(x, y, z)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::assert;

    #[test]
    fn test_mut_field() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a.y = 20.0;
        assert_eq!(a, Vec3::new(1.0, 20.0, 3.0));
    }

    #[test]
    #[should_panic(expected = "non-finite values not allowed")]
    fn test_infinity() {
        let a = Vec3::new(f64::INFINITY, 0.0, 0.0);
        a.validate();
    }

    #[test]
    #[should_panic(expected = "non-finite values not allowed")]
    fn test_nan() {
        let a = Vec3::new(f64::NAN, 0.0, 0.0);
        a.validate();
    }

    #[test]
    fn test_neg() {
        let a = Vec3::inew(1, 2, 3);
        let b = -a;
        assert_eq!(b, Vec3::inew(-1, -2, -3));
    }

    #[test]
    fn test_add() {
        let a = Vec3::inew(1, 2, 3);
        let b = Vec3::inew(1, 2, 3);
        let c = a + b;
        assert_eq!(c, Vec3::inew(2, 4, 6));
    }

    #[test]
    fn test_add_assign() {
        let a = Vec3::inew(1, 2, 3);
        let b = Vec3::inew(1, 2, 3);
        let mut c = a + b;
        c += a;
        assert_eq!(c, Vec3::inew(3, 6, 9));
    }

    #[test]
    fn test_sub() {
        let a = Vec3::inew(2, 4, 8);
        let b = Vec3::inew(1, 1, 1);
        let c = a - b;
        assert_eq!(c, Vec3::inew(1, 3, 7));
    }

    #[test]
    fn test_sub_assign() {
        let a = Vec3::inew(2, 4, 8);
        let b = Vec3::inew(1, 1, 1);
        let mut c = a - b;
        c -= a;
        assert_eq!(c, Vec3::inew(-1, -1, -1));
    }

    #[test]
    fn test_index() {
        let a = Vec3::inew(1, 2, 3);
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut a = Vec3::inew(1, 2, 3);
        a[0] = 4.0;
        a[1] = 5.0;
        a[2] = 6.0;
        assert_eq!(a, Vec3::inew(4, 5, 6));
    }

    #[test]
    fn test_mult_vec3() {
        let a = Vec3::inew(1, 2, 3);
        let b = Vec3::inew(2, 4, 6);
        let c = a * b;
        assert_eq!(c, Vec3::inew(2, 8, 18));
    }

    #[test]
    fn test_mult_vec3_assign() {
        let mut a = Vec3::inew(1, 2, 3);
        let b = Vec3::inew(2, 4, 6);
        a *= b;
        assert_eq!(a, Vec3::inew(2, 8, 18));
    }

    #[test]
    fn test_mult_f64() {
        let a = Vec3::inew(1, 2, 3);
        let b = a * 2.0;
        assert_eq!(b, Vec3::inew(2, 4, 6));
    }

    #[test]
    fn test_mult_f64_reverse() {
        let a = Vec3::inew(1, 2, 3);
        let b = 2.0 * a;
        assert_eq!(b, Vec3::inew(2, 4, 6));
    }

    #[test]
    fn test_mult_f64_assign() {
        let mut a = Vec3::inew(1, 2, 3);
        a *= 2.0;
        assert_eq!(a, Vec3::inew(2, 4, 6));
    }

    #[test]
    fn test_div() {
        let a = Vec3::inew(2, 4, 8);
        let b = a / 2.0;
        assert_eq!(b, Vec3::inew(1, 2, 4));
    }

    #[test]
    fn test_div_assign() {
        let mut a = Vec3::inew(2, 4, 8);
        a /= 2.0;
        assert_eq!(a, Vec3::inew(1, 2, 4));
    }

    #[test]
    #[should_panic(expected = "non-finite values not allowed")]
    fn test_div_nan() {
        let mut a = Vec3::inew(2, 4, 8);
        a /= 0.0;
        a.validate();
    }

    #[test]
    fn test_length_squared() {
        let a = Vec3::inew(1, 2, 3);
        assert_eq!(a.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let a = Vec3::inew(2, 2, 2);
        assert::float(a.length(), 3.464, 3);
    }

    #[test]
    fn test_dot() {
        let a = Vec3::inew(1, 2, 3);
        let b = Vec3::inew(2, 4, 6);
        let result = a.dot(&b);
        assert_eq!(result, 28.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::inew(1, 3, 6);
        let b = Vec3::inew(2, 4, 6);
        let result = a.cross(&b);
        assert_eq!(result, Vec3::inew(-6, 6, -2));
    }

    #[test]
    fn test_unit() {
        let a = Vec3::inew(2, 4, 6);
        let result = a.unit();
        assert::float(result.x, 0.2672612, 5);
        assert::float(result.y, 0.5345224, 5);
        assert::float(result.z, 0.8017837, 5);
    }
}
