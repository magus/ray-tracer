use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
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
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit() {
        let a = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));

        assert_eq!(a.at(0.0), Point3::new(1.0, 2.0, 3.0));
        assert_eq!(a.at(0.5), Point3::new(3.0, 4.5, 6.0));
        assert_eq!(a.at(4.0), Point3::new(17.0, 22.0, 27.0));
    }
}
