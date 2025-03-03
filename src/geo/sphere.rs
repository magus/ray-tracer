use crate::geo::hittable;
use crate::geo::material;
use crate::geo::Interval;
use crate::geo::Point3;
use crate::geo::Ray;
use crate::geo::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: material::Type,
    collision: bool,
}

pub struct SphereBuilder {
    center: Option<Point3>,
    radius: Option<f64>,
    material: Option<material::Type>,
    collision: Option<bool>,
}

impl SphereBuilder {
    pub fn build(&self) -> Sphere {
        Sphere {
            center: self.center.unwrap_or(Point3::new(0.0, 0.0, 0.0)),
            radius: self.radius.unwrap_or(0.0).max(0.0),
            material: self.material.unwrap_or(material::Type::empty()),
            collision: self.collision.unwrap_or(true),
        }
    }

    pub fn center(mut self, x: f64, y: f64, z: f64) -> Self {
        self.center = Some(Point3::new(x, y, z));
        self
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn material(mut self, material: material::Type) -> Self {
        self.material = Some(material);
        self
    }

    pub fn collision(mut self, collision: bool) -> Self {
        self.collision = Some(collision);
        self
    }
}

impl Sphere {
    pub fn builder() -> SphereBuilder {
        SphereBuilder {
            center: None,
            radius: None,
            material: None,
            collision: None,
        }
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> material::Type {
        self.material
    }

    pub fn collision(&self) -> bool {
        self.collision
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

        let mut hit_record = hittable::HitRecord {
            t: root,
            p,
            normal,
            front_face: false,
            material: self.material,
        };

        hit_record.set_face_normal(ray);

        Some(hit_record)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
        assert_eq!(sphere.material, material::Type::empty());
    }

    #[test]
    fn test_sphere_radius_minimum() {
        let sphere = Sphere::builder().radius(-10.0).build();
        assert_eq!(sphere.radius, 0.0);
    }

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::builder().center(0.0, 0.0, -1.0).radius(0.5).build();
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
        let sphere = Sphere::builder().center(0.0, 0.0, -1.0).radius(0.5).build();
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit.is_none());
    }

    #[test]
    fn test_sphere_material() {
        let material = material::Type::from(material::LambertianParams {
            albedo: Color::new(1.0, 0.0, 0.0),
            reflectance: 1.0,
            uniform: false,
        });

        let sphere = Sphere::builder()
            .center(0.0, 0.0, -1.0)
            .radius(0.5)
            .material(material)
            .build();

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit.is_some());
    }
}
