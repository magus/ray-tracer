use ray_tracer::core::Camera;
use ray_tracer::core::Color;
use ray_tracer::geo::material;
use ray_tracer::geo::HittableList;
use ray_tracer::geo::Point3;
use ray_tracer::geo::Sphere;

fn main() {
    let mut world = HittableList::new();

    let mat_ground = material::Type::from(material::LambertianParams {
        albedo: Color::new(0.8, 0.8, 0.0),
        reflectance: 1.0,
        uniform: false,
    });

    let mat_center = material::Type::from(material::LambertianParams {
        albedo: Color::new(0.1, 0.2, 0.5),
        reflectance: 1.0,
        uniform: false,
    });

    let mat_left = material::Type::from(material::DielectricParams {
        refraction_index: 1.5,
    });

    let mat_bubble = material::Type::from(material::DielectricParams {
        // air to glass
        refraction_index: 1.0 / 1.5,
    });

    let mat_right = material::Type::from(material::MetalParams {
        albedo: Color::new(0.8, 0.6, 0.2),
        reflectance: 1.0,
        fuzz: 0.4,
    });

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
        .vertical_fov(20.0)
        .look_from(-2.0, 2.0, 1.0)
        .look_at(0.0, 0.0, -1.0)
        .vup(0.0, 1.0, 0.0)
        .initialize();

    // camera.debug(&world, 100, 200);

    camera.render(&world);
}
