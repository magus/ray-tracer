use ray_tracer::color::Color;
use ray_tracer::hittable::{self, Hittable};
use ray_tracer::point3::Point3;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_height = 400.0;
    let image_width = image_height * aspect_ratio;
    let real_aspect_ratio = image_width / image_height;
    dbg!((image_width, image_height, aspect_ratio, real_aspect_ratio));

    // world
    let mut world = hittable::HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    // camera center aka eye point where all rays are cast from
    // right-handed coordinates
    // y-axis = vertical, x-axis = horizontal, z-axis orthogonal to viewport
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
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    // subtract focal to move from camera to viewport
    // subtract half viewport u + v to move from center to upper left corner
    let viewport_upper_left = Vec3::from(camera_center)
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    dbg!((viewport_upper_left, pixel_00));

    // render

    let y_max = image_height as u32;
    let x_max = image_width as u32;
    let max_value = 255;

    println!("P3");
    println!("{x_max} {y_max}");
    println!("{max_value}");

    for y in 0..y_max {
        eprint!("saving {}/{y_max}\r", y + 1);

        for x in 0..x_max {
            let pixel_center = pixel_00 + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - Vec3::from(camera_center);
            let ray = Ray::new(camera_center, ray_direction);

            let pixel = ray_color(&ray, &world);
            println!("{pixel}");
        }
    }

    eprintln!();
    eprintln!("saved");
}

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}

fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    let maybe_hit = world.hit(&ray, 0.0, f64::INFINITY);

    match maybe_hit {
        Some(hit) => {
            let n = hit.normal.unit();
            // normal is in range [-1, 1], add 1 ([0, 2]) and halving ([0, 1])
            let normal_01 = 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
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
