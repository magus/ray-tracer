use crate::geo::hittable;
use crate::geo::Interval;
use crate::geo::MaterialType;
use crate::geo::Point3;
use crate::geo::Ray;
use crate::geo::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: MaterialType,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: MaterialType) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
            material,
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
        let t_interval = Interval::new(t_min, t_max);

        let oc = Vec3::from(self.center) - Vec3::from(ray.origin());
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !t_interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_interval.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (Vec3::from(p) - Vec3::from(self.center)) / self.radius;

        let hit_record = hittable::HitRecord {
            t: root,
            p,
            normal,
            front_face: false,
            material: self.material,
        };

        hit_record.set_face_normal(ray);

        Some(hit_record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Color;
    use crate::geo::Hittable;

    #[test]
    fn test_sphere_default() {
        let sphere = <Sphere>::default();
        assert_eq!(sphere.radius, 0.0);
        assert_eq!(sphere.material, MaterialType::empty());
    }

    #[test]
    fn test_sphere_radius_minimum() {
        let material = MaterialType::empty();
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), -10.0, material);
        assert_eq!(sphere.radius, 0.0);
    }

    #[test]
    fn test_sphere_hit() {
        let material = MaterialType::empty();
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material);
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
        let material = MaterialType::empty();
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material);
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit.is_none());
    }

    #[test]
    fn test_sphere_material() {
        let material = MaterialType::lambertian(Color::new(1.0, 0.0, 0.0), 1.0, false);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit.is_some());
    }
}
