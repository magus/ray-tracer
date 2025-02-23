use crate::geo::Vec3;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point3(Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
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

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        *p
    }
}

impl From<&Point3> for Vec3 {
    fn from(p: &Point3) -> Self {
        **p
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Self {
        Point3(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let a = Point3::default();
        assert_eq!(format!("{a}"), "(0, 0, 0)");
    }

    #[test]
    fn test_display() {
        let a = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{a}"), "(1, 2, 3)");
    }

    #[test]
    fn test_from_point3() {
        let a = Point3::new(1.0, 2.0, 3.0);
        let b = Vec3::from(a);
        assert_eq!(b, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_from_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Point3::from(a);
        assert_eq!(b, Point3::new(1.0, 2.0, 3.0));
    }
}
