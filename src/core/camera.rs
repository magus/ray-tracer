use crate::core::Color;
use crate::geo::hittable;
use crate::geo::Point3;
use crate::geo::Ray;
use crate::geo::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    image_width: f64,
    image_height: f64,
    center: Point3,
    pixel_00: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

pub struct CameraBuilder {
    aspect_ratio: f64,
    image_height: f64,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_height: 100.0,
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

    pub fn initialize(&self) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_height = self.image_height;
        let image_width = image_height * aspect_ratio;

        // camera center aka eye point where all rays are cast from
        // right-handed coordinates
        // y-axis = vertical, x-axis = horizontal, z-axis orthogonal to viewport
        // use actual aspect ratio of image to calculate viewport size
        // may differ from aspect ratio due to rounding
        let real_aspect_ratio = image_width / image_height;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * real_aspect_ratio;
        dbg!((viewport_width, viewport_height));

        // vectors along viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // delta vectors between pixels
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // location of upper left pixel
        let focal_length = 1.0;
        let center = Point3::new(0.0, 0.0, 0.0);
        // subtract focal to move from camera to viewport
        // subtract half viewport u + v to move from center to upper left corner
        let viewport_upper_left = Vec3::from(center)
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        dbg!((viewport_upper_left, pixel_00));

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_00: Point3::from(pixel_00),
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}

impl Camera {
    pub fn new() -> CameraBuilder {
        CameraBuilder::new()
    }

    pub fn render<T: hittable::Hittable>(&self, world: &T) {
        let y_max = self.image_height as u32;
        let x_max = self.image_width as u32;
        let max_value = 255;

        println!("P3");
        println!("{x_max} {y_max}");
        println!("{max_value}");

        for y in 0..y_max {
            eprint!("saving {}/{y_max}\r", y + 1);

            for x in 0..x_max {
                let pixel_center = Vec3::from(self.pixel_00)
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - Vec3::from(self.center);
                let ray = Ray::new(self.center, ray_direction);

                let pixel = ray_color(&ray, world);
                println!("{pixel}");
            }
        }

        eprintln!();
        eprintln!("saved");
    }
}

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}

fn ray_color<T: hittable::Hittable>(ray: &Ray, world: &T) -> Color {
    let maybe_hit = world.hit(&ray, 0.0, f64::INFINITY);

    match maybe_hit {
        Some(hit) => {
            // normal is in range [-1, 1], add 1 ([0, 2]) and halving ([0, 1])
            let normal_01 = 0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0));
            return Color::from(normal_01);
        }
        _ => {
            let unit_direction = ray.direction().unit();
            let a = 0.5 * (unit_direction.y() + 1.0);
            let white = Color::new(1.0, 1.0, 1.0);
            let blue = Color::new(0.5, 0.7, 1.0);

            // Color::new(0.0, 0.0, 0.0)
            Color::from(lerp(a, white.into(), blue.into()))
        }
    }
}
