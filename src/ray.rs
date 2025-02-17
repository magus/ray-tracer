use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        Point3::from(Vec3::from(self.origin) + t * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let a = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        let b = Ray::new(*a.origin(), *a.direction());
        assert_eq!(b, a);
    }

    #[test]
    fn test_mut_field_copy() {
        let a = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        let mut dir = *a.direction();
        dir.x = 10.0;
        assert_eq!(dir, Vec3::new(10.0, 5.0, 6.0));
    }

    #[test]
    fn test_at() {
        let a = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));

        assert_eq!(a.at(0.0), Point3::new(1.0, 2.0, 3.0));
        assert_eq!(a.at(0.5), Point3::new(3.0, 4.5, 6.0));
        assert_eq!(a.at(4.0), Point3::new(17.0, 22.0, 27.0));
    }
}
