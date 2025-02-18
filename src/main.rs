use ray_tracer::core::Camera;
use ray_tracer::geo::hittable;
use ray_tracer::geo::Point3;
use ray_tracer::geo::Sphere;

fn main() {
    let mut world = hittable::HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new()
        .aspect_ratio(16.0 / 9.0)
        .image_height(400)
        .samples_per_pixel(10)
        .max_depth(50)
        .initialize();

    camera.render(&world);
}
