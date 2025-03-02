use ray_tracer::core::Camera;
use ray_tracer::core::Color;
use ray_tracer::geo::material;
use ray_tracer::geo::HittableList;
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
        albedo: Color::new(0.75, 0.75, 0.75),
        reflectance: 1.0,
        fuzz: 0.3,
    });

    world.add(
        Sphere::builder()
            .center(0.0, -100.5, -1.0)
            .radius(100.0)
            .material(mat_ground)
            .build(),
    );

    world.add(
        Sphere::builder()
            .center(0.0, 0.0, -1.2)
            .radius(0.5)
            .material(mat_center)
            .build(),
    );

    world.add(
        Sphere::builder()
            .center(-1.0, 0.0, -1.0)
            .radius(0.5)
            .material(mat_left)
            .build(),
    );

    world.add(
        Sphere::builder()
            .center(-1.0, 0.0, -1.0)
            .radius(0.4)
            .material(mat_bubble)
            .build(),
    );

    world.add(
        Sphere::builder()
            .center(1.0, 0.0, -1.0)
            .radius(0.5)
            .material(mat_right)
            .build(),
    );

    let camera = Camera::new()
        .aspect_ratio(16.0 / 9.0)
        .image_height(400)
        .samples_per_pixel(10)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(-2.0, 2.0, 1.0)
        .look_at(0.0, 0.0, -1.0)
        .vup(0.0, 1.0, 0.0)
        .defocus_angle(0.0)
        .focus_distance(10.0)
        .initialize();

    // camera.debug(&world, 100, 200);

    camera.render(&world);
}
