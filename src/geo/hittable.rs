use crate::geo::Interval;
use crate::geo::MaterialType;
use crate::geo::Point3;
use crate::geo::Ray;
use crate::geo::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: MaterialType,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray) {
        // NOTE: `normal` assumed to have unit length

        self.front_face = ray.direction().dot(&self.normal) < 0.0;

        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t_interval = Interval::new(t_min, t_max);

        let mut closest_so_far = t_interval.max();
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}
