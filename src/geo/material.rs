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
pub enum Type {
    Empty(Empty),
    Debug(Debug),
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Default for Type {
    fn default() -> Self {
        Type::Empty(Empty {})
    }
}

pub enum Params {
    Lambertian(LambertianParams),
    Metal(MetalParams),
    Dielectric(DielectricParams),
}

impl From<LambertianParams> for Params {
    fn from(p: LambertianParams) -> Self {
        Params::Lambertian(p)
    }
}

impl From<MetalParams> for Params {
    fn from(p: MetalParams) -> Self {
        Params::Metal(p)
    }
}

impl From<DielectricParams> for Params {
    fn from(p: DielectricParams) -> Self {
        Params::Dielectric(p)
    }
}

impl Type {
    // Helper constructors for external use.
    pub fn empty() -> Self {
        Type::Empty(Empty {})
    }

    pub fn debug() -> Self {
        Type::Debug(Debug {})
    }

    pub fn from<P>(params: P) -> Self
    where
        P: Into<Params>,
    {
        match params.into() {
            Params::Lambertian(params) => {
                return Type::Lambertian(Lambertian {
                    albedo: params.albedo,
                    reflectance: params.reflectance,
                    uniform: params.uniform,
                })
            }

            Params::Metal(params) => {
                return Type::Metal(Metal {
                    albedo: params.albedo,
                    reflectance: params.reflectance,
                    fuzz: params.fuzz.min(1.0),
                })
            }

            Params::Dielectric(params) => {
                return Type::Dielectric(Dielectric {
                    refraction_index: params.refraction_index,
                })
            }
        }
    }
}

impl Type {
    pub fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<ScatterRecord> {
        match self {
            Type::Empty(m) => m.scatter(ray, hit),
            Type::Debug(m) => m.scatter(ray, hit),
            Type::Lambertian(m) => m.scatter(ray, hit),
            Type::Metal(m) => m.scatter(ray, hit),
            Type::Dielectric(m) => m.scatter(ray, hit),
        }
    }
}

pub struct LambertianParams {
    pub albedo: Color,
    pub reflectance: f64,
    pub uniform: bool,
}

impl Default for LambertianParams {
    fn default() -> Self {
        Self {
            albedo: Color::new(1.0, 0.0, 0.0),
            reflectance: 1.0,
            uniform: false,
        }
    }
}

pub struct MetalParams {
    pub albedo: Color,
    pub reflectance: f64,
    pub fuzz: f64,
}

impl Default for MetalParams {
    fn default() -> Self {
        Self {
            albedo: Color::new(1.0, 0.0, 0.0),
            reflectance: 1.0,
            fuzz: 0.0,
        }
    }
}

pub struct DielectricParams {
    pub refraction_index: f64,
}

impl Default for DielectricParams {
    fn default() -> Self {
        Self {
            refraction_index: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Color,
    pub color: Option<Color>,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: HitRecord) -> Option<ScatterRecord>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Empty {}

impl Material for Empty {
    fn scatter(&self, _ray_in: &Ray, _hit_record: HitRecord) -> Option<ScatterRecord> {
        None
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Debug {}

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
