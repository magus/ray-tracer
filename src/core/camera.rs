use crate::core::random_f64;
use crate::core::Color;
use crate::geo::Hittable;
use crate::geo::Point3;
use crate::geo::Ray;
use crate::geo::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    image_width: f64,
    image_height: f64,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
    center: Point3,
    pixel_00: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

pub struct CameraBuilder {
    aspect_ratio: f64,
    image_height: f64,
    /// Count of random samples for each pixel
    samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene
    max_depth: u32,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_height: 100.0,
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> CameraBuilder {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn image_height(mut self, image_height: u32) -> CameraBuilder {
        self.image_height = image_height as f64;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> CameraBuilder {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> CameraBuilder {
        self.max_depth = max_depth;
        self
    }

    pub fn initialize(&self) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_height = self.image_height;
        let image_width = image_height * aspect_ratio;

        let samples_per_pixel = self.samples_per_pixel;
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let max_depth = self.max_depth;

        // camera center aka eye point where all rays are cast from
        // right-handed coordinates
        // y-axis = vertical, x-axis = horizontal, z-axis orthogonal to viewport
        // use actual aspect ratio of image to calculate viewport size
        // may differ from aspect ratio due to rounding
        let real_aspect_ratio = image_width / image_height;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * real_aspect_ratio;

        // dbg!((viewport_width, viewport_height));

        // vectors along viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // delta vectors between pixels
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let focal_length = 1.0;
        let center = Point3::new(0.0, 0.0, 0.0);

        // location of upper left pixel
        // subtract focal to move from camera to viewport
        // subtract half viewport u + v to move from center to upper left corner
        let viewport_upper_left = Vec3::from(center)
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel_00 = Point3::from(viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v));

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            center,
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}

impl Camera {
    pub fn new() -> CameraBuilder {
        CameraBuilder::new()
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        let y_max = self.image_height as u32;
        let x_max = self.image_width as u32;
        let max_value = 255;

        println!("P3");
        println!("{x_max} {y_max}");
        println!("{max_value}");

        for y in 0..y_max {
            eprint!("\rsaving {}/{y_max}", y + 1);

            for x in 0..x_max {
                let mut pixel_vec3 = Vec3::from(Color::new(0.0, 0.0, 0.0));

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    let color = ray_color(&ray, world, self.max_depth);
                    pixel_vec3 += Vec3::from(color);
                }

                let pixel_vec3 = pixel_vec3 * self.pixel_samples_scale;
                let pixel = Color::from(pixel_vec3);
                println!("{pixel}");
            }
        }

        eprintln!();
        eprintln!("saved");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        // ray originating from camera center directed at
        // randomly sampled point around pixel (x, y)
        let offset = sample_square();

        let pixel_sample = Vec3::from(self.pixel_00)
            + ((x as f64 + offset.x()) * self.pixel_delta_u)
            + ((y as f64 + offset.y()) * self.pixel_delta_v);

        let ray_direction = pixel_sample - Vec3::from(self.center);
        Ray::new(self.center, ray_direction)
    }
}

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}

fn ray_color<T: Hittable>(ray: &Ray, world: &T, depth: u32) -> Color {
    // exceeded ray bounce limit, stop gathering light
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // lower bound t=0.001 to avoid self-intersect near surface
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scatter_record) = hit.material.scatter(ray, hit) {
            // early return if color is provided, e.g. Debug material
            if let Some(color) = scatter_record.color {
                return color;
            }

            let attentuation = Vec3::from(scatter_record.attenuation);
            let next_ray_color = Vec3::from(ray_color(&scatter_record.ray, world, depth - 1));
            return Color::from(attentuation * next_ray_color);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);

    // Color::new(0.0, 0.0, 0.0)
    Color::from(lerp(a, white.into(), blue.into()))
}

fn sample_square() -> Point3 {
    // random point in the [-0.5,-0.5] [+0.5,+0.5] unit square
    Point3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
}
