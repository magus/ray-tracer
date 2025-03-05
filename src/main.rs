use ray_tracer::core::ppm;
use ray_tracer::core::random_f64;
use ray_tracer::core::random_f64_range;
use ray_tracer::core::Camera;
use ray_tracer::core::Color;
use ray_tracer::geo::material;
use ray_tracer::geo::HittableList;
use ray_tracer::geo::Sphere;
use ray_tracer::geo::Vec3;

#[tokio::main]
async fn main() {
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

    let radius = random_f64_range(1.2, 1.4);
    world.add(
        Sphere::builder()
            .center(-4.0, radius, 0.0)
            .radius(radius)
            .material(material::Type::from(material::LambertianParams {
                albedo: Color::new(0.4, 0.2, 0.1),
                reflectance: 1.0,
                uniform: false,
            }))
            .build(),
    );

    let radius = random_f64_range(1.0, 1.2);
    world.add(
        Sphere::builder()
            .center(0.0, radius, 0.0)
            .radius(radius)
            .material(material::Type::from(material::DielectricParams {
                refraction_index: 1.5,
            }))
            .build(),
    );

    let radius = random_f64_range(0.8, 1.0);
    world.add(
        Sphere::builder()
            .center(4.0, radius, 0.0)
            .radius(radius)
            .material(material::Type::from(material::MetalParams {
                albedo: Color::new(0.7, 0.6, 0.5),
                reflectance: 1.0,
                fuzz: 0.0,
            }))
            .build(),
    );

    let item_count = 11;
    let min_distance_multiplier = 1.0;

    for x in -item_count..item_count {
        for z in -item_count..item_count {
            let sphere = random_sphere(RandomSphereParams {
                x: x as f64,
                z: z as f64,
                glass_chance: 0.4,
                metal_chance: 0.3,
                lambertian_chance: 0.3,
            });

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
        .image_height(1080)
        // .image_height(540)
        // .image_height(270)
        // .image_height(100)
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
    let pixels = camera.render(&world);

    let ppm = ppm::V3 {
        width: camera.image_width(),
        height: camera.image_height(),
        pixels,
    };

    if let Err(error) = ppm.save("image.ppm").await {
        eprintln!("{error}");
    };
}

struct RandomSphereParams {
    x: f64,
    z: f64,
    glass_chance: f64,
    metal_chance: f64,
    lambertian_chance: f64,
}
fn random_sphere(params: RandomSphereParams) -> Sphere {
    let radius = random_f64_range(0.1, 0.3);

    let center = Vec3::new(
        params.x + 0.9 * random_f64(),
        radius,
        params.z + 0.9 * random_f64(),
    );

    let glass_chance = 1.0 - params.glass_chance;
    let metal_chance = glass_chance - params.metal_chance;
    let lambertian_chance = metal_chance - params.lambertian_chance;

    // dbg!(glass_chance, metal_chance, lambertian_chance);

    let material_chance = random_f64();

    let material = if material_chance > glass_chance {
        material::Type::from(material::DielectricParams {
            refraction_index: 1.5,
        })
    } else if material_chance > metal_chance {
        material::Type::from(material::MetalParams {
            albedo: Color::from(Vec3::random_range(0.5, 1.0)),
            reflectance: 1.0,
            fuzz: random_f64_range(0.0, 0.5),
        })
    } else if material_chance > lambertian_chance {
        material::Type::from(material::LambertianParams {
            albedo: Color::from(Vec3::random() * Vec3::random()),
            reflectance: 1.0,
            uniform: false,
        })
    } else {
        panic!("material chance not handled ({material_chance})");
    };

    Sphere::builder()
        .center(center.x, center.y, center.z)
        .radius(radius)
        .material(material)
        .build()
}
