use ray_tracer::core::random_f64;
use ray_tracer::core::random_f64_range;
use ray_tracer::core::Camera;
use ray_tracer::core::Color;
use ray_tracer::geo::material;
use ray_tracer::geo::HittableList;
use ray_tracer::geo::Sphere;
use ray_tracer::geo::Vec3;

fn main() {
    let mut world = HittableList::new();

    let ground_radius = 1000.0;

    // ground
    world.add(
        Sphere::builder()
            .center(0.0, -ground_radius, 0.0)
            .radius(ground_radius)
            .material(material::Type::from(material::LambertianParams {
                albedo: Color::new(0.5, 0.5, 0.5),
                reflectance: 1.0,
                uniform: false,
            }))
            .collision(false)
            .build(),
    );

    world.add(
        Sphere::builder()
            .center(-4.0, 1.0, 0.0)
            .radius(1.0)
            .material(material::Type::from(material::LambertianParams {
                albedo: Color::new(0.4, 0.2, 0.1),
                reflectance: 1.0,
                uniform: false,
            }))
            .build(),
    );

    world.add(
        Sphere::builder()
            .center(0.0, 1.0, 0.0)
            .radius(1.0)
            .material(material::Type::from(material::DielectricParams {
                refraction_index: 1.5,
            }))
            .build(),
    );

    world.add(
        Sphere::builder()
            .center(4.0, 1.0, 0.0)
            .radius(1.0)
            .material(material::Type::from(material::MetalParams {
                albedo: Color::new(0.7, 0.6, 0.5),
                reflectance: 1.0,
                fuzz: 0.0,
            }))
            .build(),
    );

    let item_count = 22;
    let min_distance_multiplier = 1.0;

    for x in -item_count..item_count {
        for z in -item_count..item_count {
            let sphere = random_sphere(x, z);

            let mut include = true;

            // only add sphere if its far enough away from existing larger spheres
            for object in world.objects() {
                // detect object is sphere
                if let Some(object_sphere) = object.as_any().downcast_ref::<Sphere>() {
                    // skip spheres that are not collision (i.e. ground)
                    if !object_sphere.collision() {
                        continue;
                    }

                    // only add sphere if its far enough away from other spheres
                    // adjust object_center to match the y level of generated sphere
                    // so our distance formula is calculating the distance at same y level
                    let radius = sphere.radius();
                    let mut object_center = Vec3::from(object_sphere.center());
                    object_center.y = radius;

                    let distance = (Vec3::from(sphere.center()) - object_center).length();

                    // min distance to ensure spheres do not overlap
                    let sphere_distance = sphere.radius() + object_sphere.radius();
                    let min_distance = sphere_distance * min_distance_multiplier;

                    if distance < min_distance {
                        include = false;
                    }

                    // dbg!((min_distance, sphere_distance, distance, include));
                }
            }

            if include {
                world.add(sphere);
            }
        }
    }

    let camera = Camera::new()
        .aspect_ratio(16.0 / 9.0)
        .image_height(1200)
        .samples_per_pixel(10)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(13.0, 2.0, 3.0)
        .look_at(0.0, 0.0, 0.0)
        .vup(0.0, 1.0, 0.0)
        .defocus_angle(0.6)
        .focus_distance(10.0)
        .initialize();

    // camera.debug(&world, 100, 200);
    camera.render(&world);
}

fn random_sphere(x: i32, z: i32) -> Sphere {
    let radius = random_f64_range(0.1, 0.3);

    let choice = random_f64();

    let center = Vec3::new(
        x as f64 + 0.9 * random_f64(),
        radius,
        z as f64 + 0.9 * random_f64(),
    );

    let material = if choice > 0.95 {
        // glass
        material::Type::from(material::DielectricParams {
            refraction_index: 1.5,
        })
    } else if choice > 0.8 {
        // metal
        material::Type::from(material::MetalParams {
            albedo: Color::from(Vec3::random_range(0.5, 1.0)),
            reflectance: 1.0,
            fuzz: random_f64_range(0.0, 0.5),
        })
    } else {
        // diffuse
        // auto albedo = color::random() * color::random();
        // sphere_material = make_shared<lambertian>(albedo);
        material::Type::from(material::LambertianParams {
            albedo: Color::from(Vec3::random() * Vec3::random()),
            reflectance: 1.0,
            uniform: false,
        })
    };

    Sphere::builder()
        .center(center.x, center.y, center.z)
        .radius(radius)
        .material(material)
        .build()
}
