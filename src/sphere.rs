use crate::hittable::{self, HitRecord};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
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

        Some(HitRecord {
            t: root,
            p,
            normal: (Vec3::from(p) - Vec3::from(self.center)) / self.radius,
        })
    }
}
