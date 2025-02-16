use crate::vec3::Vec3;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3(Vec3);

impl Point3 {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Point3(Vec3::new(x, y, z))
    }
}

impl std::ops::Deref for Point3 {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let a = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{a}"), "(1, 2, 3)");
    }
}
