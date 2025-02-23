use crate::core::random_f64;
use crate::core::Color;
use crate::geo::random_unit;
use crate::geo::random_unit_normal_direction;
use crate::geo::HitRecord;
use crate::geo::Ray;
use crate::geo::Vec3;

// using an enum here for compile time known sizing so we
// can use it in a struct without awkward Box or lifetimes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialType {
    Empty(Empty),
    Debug(Debug),
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MaterialType {
    // Helper constructors for external use.
    pub fn empty() -> Self {
        MaterialType::Empty(Empty::new())
    }

    pub fn debug() -> Self {
        MaterialType::Debug(Debug::new())
    }

    pub fn lambertian(albedo: Color, reflectance: f64) -> Self {
        MaterialType::Lambertian(Lambertian::with_reflectance(albedo, reflectance))
    }

    pub fn metal(albedo: Color, reflectance: f64) -> Self {
        MaterialType::Metal(Metal::with_reflectance(albedo, reflectance))
    }
}

impl MaterialType {
    pub fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<ScatterRecord> {
        match self {
            MaterialType::Empty(m) => m.scatter(ray, hit),
            MaterialType::Debug(m) => m.scatter(ray, hit),
            MaterialType::Lambertian(m) => m.scatter(ray, hit),
            MaterialType::Metal(m) => m.scatter(ray, hit),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Color,
    pub color: Option<Color>,
}

impl ScatterRecord {}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord) -> Option<ScatterRecord>;
}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::Empty(Empty::new())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Empty {
        Empty {}
    }
}

impl Material for Empty {
    fn scatter(&self, _ray_in: &Ray, _hit_record: HitRecord) -> Option<ScatterRecord> {
        None
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Debug {}

impl Debug {
    pub fn new() -> Debug {
        Debug {}
    }
}

impl Material for Debug {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        // color based on normal
        // normal is in range [-1, 1], add 1 ([0, 2]) and halving ([0, 1])
        let normal_01 = 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
        let color = Color::from(normal_01);

        Some(ScatterRecord {
            ray: *ray_in,
            attenuation: Color::new(0.0, 0.0, 0.0),
            color: Some(color),
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Lambertian {
    // albedo is latin for 'whiteness' or 'fractional reflectance'
    albedo: Color,
    // reflectance is the fraction of incident light that is reflected
    // 0 all light absorbed, 1 all light reflected
    reflectance: f64,
    uniform: bool,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        let reflectance = 1.0;
        Lambertian::with_reflectance(albedo, reflectance)
    }

    pub fn with_reflectance(albedo: Color, reflectance: f64) -> Lambertian {
        Lambertian {
            albedo,
            reflectance,
            uniform: false,
        }
    }

    pub fn uniform(albedo: Color, reflectance: f64) -> Lambertian {
        Lambertian {
            albedo,
            reflectance,
            uniform: true,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let direction = if self.uniform {
            // uniform distribution of rays
            random_unit_normal_direction(&hit_record.normal)
        } else {
            let mut direction = hit_record.normal + random_unit();

            if direction.near_zero() {
                direction = hit_record.normal;
            }

            direction
        };

        reflectance_scatter(ReflectanceScatterOptions {
            hit_record,
            direction,
            albedo: self.albedo,
            reflectance: self.reflectance,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metal {
    // albedo is latin for 'whiteness' or 'fractional reflectance'
    albedo: Color,
    // reflectance is the fraction of incident light that is reflected
    // 0 all light absorbed, 1 all light reflected
    reflectance: f64,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        let reflectance = 1.0;
        Metal::with_reflectance(albedo, reflectance)
    }

    pub fn with_reflectance(albedo: Color, reflectance: f64) -> Metal {
        Metal {
            albedo,
            reflectance,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let direction = ray_in.direction().reflect(&hit_record.normal);

        reflectance_scatter(ReflectanceScatterOptions {
            hit_record,
            direction,
            albedo: self.albedo,
            reflectance: self.reflectance,
        })
    }
}

pub struct ReflectanceScatterOptions {
    hit_record: HitRecord,
    direction: Vec3,
    albedo: Color,
    reflectance: f64,
}

fn reflectance_scatter(options: ReflectanceScatterOptions) -> Option<ScatterRecord> {
    // either randomly scatter a ray with probability p, or absorb it with probability 1 - p
    // e.g. 0.1 reflectance, very low near total black void
    // 10% chance to reflect light, 90% chance to absorb light
    // random f64 greater than reflectance is absorbed
    if random_f64() > options.reflectance {
        return None;
    }

    let scattered_ray = Ray::new(options.hit_record.p, options.direction);

    // divide by zero impossible, options.reflectance will never be zero
    // zero values are handled above since they they will always return None
    let attenuation = Color::from(Vec3::from(options.albedo) / options.reflectance);

    Some(ScatterRecord {
        ray: scattered_ray,
        attenuation,
        color: None,
    })
}
