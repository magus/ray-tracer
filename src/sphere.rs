use crate::hittable::{self, HitRecord};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl hittable::Hittable for Sphere {
    // circle hit test relies on observation that equation of sphere can be rewritten as dot product
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere/ray-sphereintersection
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let oc = Vec3::from(self.center) - Vec3::from(ray.origin());
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;

        if root <= t_min || t_max <= root {
            let root = (h + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (Vec3::from(p) - Vec3::from(self.center)) / self.radius;

        let hit_record = HitRecord {
            t: root,
            p,
            normal,
            front_face: false,
        };

        hit_record.set_face_normal(ray);

        Some(hit_record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hittable::Hittable;

    #[test]
    fn test_sphere_radius_minimum() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), -10.0);
        assert_eq!(sphere.radius, 0.0);
    }

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit.is_some());
        let record = hit.unwrap();
        assert_eq!(record.t, 0.5);
        assert_eq!(record.p, Point3::new(0.0, 0.0, -0.5));
        assert_eq!(record.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_sphere_miss() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit.is_none());
    }
}
