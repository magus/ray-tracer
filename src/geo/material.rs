use crate::core::random_f64;
use crate::core::Color;
use crate::geo::random_unit;
use crate::geo::random_unit_normal_direction;
use crate::geo::HitRecord;
use crate::geo::Ray;
use crate::geo::Vec3;

// using an enum here for compile time known sizing so we
// can use it in a struct without awkward Box or lifetimes
#[allow(private_interfaces)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialType {
    Empty(Empty),
    Debug(Debug),
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl MaterialType {
    // Helper constructors for external use.
    pub fn empty() -> Self {
        MaterialType::Empty(Empty::new())
    }

    pub fn debug() -> Self {
        MaterialType::Debug(Debug::new())
    }

    pub fn lambertian(albedo: Color, reflectance: f64, uniform: bool) -> Self {
        MaterialType::Lambertian(Lambertian::new(albedo, reflectance, uniform))
    }

    pub fn metal(albedo: Color, reflectance: f64, fuzz: f64) -> Self {
        MaterialType::Metal(Metal::new(albedo, reflectance, fuzz))
    }

    pub fn dielectric(refraction_index: f64) -> Self {
        MaterialType::Dielectric(Dielectric::new(refraction_index))
    }
}

impl MaterialType {
    pub fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<ScatterRecord> {
        match self {
            MaterialType::Empty(m) => m.scatter(ray, hit),
            MaterialType::Debug(m) => m.scatter(ray, hit),
            MaterialType::Lambertian(m) => m.scatter(ray, hit),
            MaterialType::Metal(m) => m.scatter(ray, hit),
            MaterialType::Dielectric(m) => m.scatter(ray, hit),
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
struct Empty {}

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
struct Debug {}

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
struct Lambertian {
    // albedo is latin for 'whiteness' or 'fractional reflectance'
    albedo: Color,
    // reflectance is the fraction of incident light that is reflected
    // 0 all light absorbed, 1 all light reflected
    reflectance: f64,
    // use a random unit vector instead of a random unit vector added to surface normal
    uniform: bool,
}

impl Lambertian {
    pub fn new(albedo: Color, reflectance: f64, uniform: bool) -> Lambertian {
        Lambertian {
            albedo,
            reflectance,
            uniform,
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
            fuzz: 0.0,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Metal {
    // albedo is latin for 'whiteness' or 'fractional reflectance'
    albedo: Color,
    // reflectance is the fraction of incident light that is reflected
    // 0 all light absorbed, 1 all light reflected
    reflectance: f64,
    // randomize reflected direction by using small sphere centered on the original
    // endpoint choosing a random point from the surface of the sphere
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, reflectance: f64, fuzz: f64) -> Metal {
        Metal {
            albedo,
            reflectance,
            fuzz: fuzz.min(1.0),
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
            fuzz: self.fuzz,
        })
    }
}

// clear materials such as water, glass, and diamond
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Dielectric {
    // refraction_index is the ratio of incident medium over transmitted medium
    // refraction index of material over refraction index of enclosing media
    // snell's law https://en.wikipedia.org/wiki/Snell%27s_law
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let refraction_index = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let incident_uv = ray_in.direction().unit();

        // eprintln!("Dielectric.scatter:");
        // eprintln!("  hit point={:?}", hit_record.p);
        // eprintln!("  normal={:?}", hit_record.normal);
        // eprintln!("  front_face={:?}", hit_record.front_face);
        // eprintln!("  incident direction (unit)={:?}", incident_uv);
        // eprintln!("  refraction_index={:?}", refraction_index);

        let cos_theta = incident_uv.cos_theta(&hit_record.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1.0;

        let reflectance_chance = reflectance(cos_theta, refraction_index);
        let must_reflect = reflectance_chance > random_f64();

        let direction = if cannot_refract || must_reflect {
            incident_uv.reflect(&hit_record.normal)
        } else {
            incident_uv.refract(&hit_record.normal, refraction_index)
        };

        let ray = Ray::new(hit_record.p, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);

        Some(ScatterRecord {
            ray,
            attenuation,
            color: None,
        })
    }
}

pub struct ReflectanceScatterOptions {
    hit_record: HitRecord,
    direction: Vec3,
    albedo: Color,
    reflectance: f64,
    fuzz: f64,
}

fn reflectance_scatter(options: ReflectanceScatterOptions) -> Option<ScatterRecord> {
    // either randomly scatter a ray with probability p, or absorb it with probability 1 - p
    // e.g. 0.1 reflectance, very low near total black void
    // 10% chance to reflect light, 90% chance to absorb light
    // random f64 greater than reflectance is absorbed
    if random_f64() > options.reflectance {
        return None;
    }

    let reflected = options.direction.unit() + (options.fuzz * random_unit());
    let scattered_ray = Ray::new(options.hit_record.p, reflected);

    // if the scattered ray is below the surface, it is absorbed
    let outward_component = scattered_ray.direction().dot(&options.hit_record.normal);
    if outward_component < 0.0 {
        return None;
    }

    // divide by zero impossible, options.reflectance will never be zero
    // zero values are handled above since they they will always return None
    let attenuation = Color::from(Vec3::from(options.albedo) / options.reflectance);

    Some(ScatterRecord {
        ray: scattered_ray,
        attenuation,
        color: None,
    })
}

// reflectivity varies with angle, use cheap accurate polynomial approximation
// https://en.wikipedia.org/wiki/Schlick%27s_approximation
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
