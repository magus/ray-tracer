use ray_tracer::core::Camera;
use ray_tracer::core::Color;
use ray_tracer::geo::HittableList;
use ray_tracer::geo::MaterialType;
use ray_tracer::geo::Point3;
use ray_tracer::geo::Sphere;

fn main() {
    let mut world = HittableList::new();

    let mat_ground = MaterialType::lambertian(Color::new(0.8, 0.8, 0.0), 1.0, false);
    let mat_center = MaterialType::lambertian(Color::new(0.1, 0.2, 0.5), 1.0, false);
    let mat_left = MaterialType::dielectric(1.5);
    let mat_bubble = MaterialType::dielectric(1.0 / 1.5);
    let mat_right = MaterialType::metal(Color::new(0.8, 0.6, 0.2), 1.0, 1.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        mat_center,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        mat_bubble,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    let camera = Camera::new()
        .aspect_ratio(16.0 / 9.0)
        .image_height(400)
        .samples_per_pixel(10)
        .max_depth(50)
        .initialize();

    // camera.debug(&world, 100, 200);

    camera.render(&world);
}
