use crate::core::random_f64;
use crate::core::Color;
use crate::core::Progress;
use crate::geo::degrees_to_radians;
use crate::geo::random_unit_disk;
use crate::geo::Hittable;
use crate::geo::Point3;
use crate::geo::Ray;
use crate::geo::Vec3;
use rayon::prelude::*;

pub struct CameraBuilder {
    aspect_ratio: f64,
    image_height: f64,
    /// Count of random samples for each pixel
    samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene
    max_depth: u32,
    /// Vertical view angle (field of view, fov)
    vertical_fov: f64,
    /// Point camera is looking from
    look_from: Point3,
    /// Point camera is looking at
    look_at: Point3,
    /// Camera-relative "up" direction
    vup: Vec3,
    /// Variation angle of rays through each pixel
    defocus_angle: f64,
    /// Distance from camera lookfrom point to plane of perfect focus
    focus_distance: f64,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_height: 100.0,
            samples_per_pixel: 10,
            max_depth: 10,
            vertical_fov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
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

    pub fn vertical_fov(mut self, vertical_fov: f64) -> CameraBuilder {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn look_from(mut self, x: f64, y: f64, z: f64) -> CameraBuilder {
        self.look_from = Point3::new(x, y, z);
        self
    }

    pub fn look_at(mut self, x: f64, y: f64, z: f64) -> CameraBuilder {
        self.look_at = Point3::new(x, y, z);
        self
    }

    pub fn vup(mut self, x: f64, y: f64, z: f64) -> CameraBuilder {
        self.vup = Vec3::new(x, y, z);
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f64) -> CameraBuilder {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_distance(mut self, focus_distance: f64) -> CameraBuilder {
        self.focus_distance = focus_distance;
        self
    }

    pub fn initialize(&self) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_height = self.image_height;
        let image_width = image_height * aspect_ratio;

        let samples_per_pixel = self.samples_per_pixel;
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let max_depth = self.max_depth;

        // use vertical fov to calculate viewport height
        let camera_delta_v = Vec3::from(self.look_from) - Vec3::from(self.look_at);
        let theta = degrees_to_radians(self.vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;

        // camera center aka eye point where all rays are cast from
        // right-handed coordinates
        // y-axis = vertical, x-axis = horizontal, z-axis orthogonal to viewport
        // use actual aspect ratio of image to calculate viewport size
        // may differ from aspect ratio due to rounding
        let real_aspect_ratio = image_width / image_height;
        let viewport_width = viewport_height * real_aspect_ratio;

        // dbg!((viewport_width, viewport_height));

        // calculate u,v,w unit basis vectors for camera coordinate frame
        let w = camera_delta_v.unit();
        let u = self.vup.cross(&w).unit();
        let v = w.cross(&u);

        // vectors along viewport edges
        // vector across viewport horizontal edge
        let viewport_u = viewport_width * u;
        // vector down viewport vertical edge
        let viewport_v = viewport_height * -v;

        // delta vectors between pixels
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the camera defocus disk basis vectors.
        let defocus_angle = self.defocus_angle;
        let defocus_radius = self.focus_distance * (degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let center = Vec3::from(self.look_from);

        // location of upper left pixel
        // subtract focal to move from camera to viewport
        // subtract half viewport u + v to move from center to upper left corner
        let viewport_upper_left =
            center - (self.focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

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
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    image_width: f64,
    image_height: f64,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
    center: Vec3,
    pixel_00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> CameraBuilder {
        CameraBuilder::new()
    }

    pub fn debug<T: Hittable>(&self, world: &T, x: u32, y: u32) {
        let ray = self.get_ray(x, y);
        let color = ray_color(&ray, world, self.max_depth);
        eprintln!("ray={:?}", ray);
        eprintln!("color={:?}", color);
    }

    pub fn render<T: Hittable>(&self, world: &T, pixels: &mut Vec<Color>) {
        // wrap render in block so it drops progress thread correctly
        // printing the final progress bar update before saved message
        {
            let progress = Progress::new(pixels.len());
            let _progress_thread = progress.render(15);

            pixels
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| {
                    let y = (index / self.image_width()) as u32;
                    let x = (index % self.image_width()) as u32;

                    let mut pixel_vec3 = Vec3::from(Color::new(0.0, 0.0, 0.0));

                    for _sample in 0..self.samples_per_pixel {
                        let ray = self.get_ray(x, y);
                        let color = ray_color(&ray, world, self.max_depth);
                        pixel_vec3 += Vec3::from(color);
                    }

                    let pixel_vec3 = pixel_vec3 * self.pixel_samples_scale;

                    // assign pixel color to output pixel at index
                    *pixel = Color::from(pixel_vec3);

                    // row done, update progress
                    progress.inc();
                });
        }
    }

    pub fn image_width(&self) -> usize {
        self.image_width as usize
    }

    pub fn image_height(&self) -> usize {
        self.image_height as usize
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        // ray originating from defocus disk and directed
        // at a randomly sampled point around pixel (x, y)
        let offset = sample_square();

        let pixel_sample = self.pixel_00
            + ((x as f64 + offset.x()) * self.pixel_delta_u)
            + ((y as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        Ray::new(Point3::from(ray_origin), ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_unit_disk();
        let defocus_p = self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
        defocus_p
    }
}

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}

fn ray_color<T: Hittable>(ray: &Ray, world: &T, depth: u32) -> Color {
    // eprintln!("ray_color: depth={depth}, ray={:?}", ray);

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
