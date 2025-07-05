use std::sync::Arc;

use rand::random;
use ray_tracing::Colour;
use ray_tracing::camera::Camera;
use ray_tracing::hittable::Hittable;
use ray_tracing::hittable_list::*;
use ray_tracing::material::Dielectric;
use ray_tracing::material::Lambertian;
use ray_tracing::material::Light;
use ray_tracing::material::Material;
use ray_tracing::material::Metal;
use ray_tracing::sphere::*;
use ray_tracing::vector::*;

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        1280,
        500,
        500,
        20.0,
        Vector::new(13.0, 2.0, 3.0),
        Vector::new(0.0, 0.0, 0.0),
        0.6,
        10.0,
    );

    let mut world = HittableList::new();

    // Creating the small balls in the render
    for i in -10..=10 {
        for j in -10..=10 {
            let centre = Point::new(
                f64::from(i) + 0.9 * random::<f64>(),
                0.2,
                f64::from(j) + 0.9 * random::<f64>(),
            );

            if (centre - Point::new(4.0, 0.2, 0.0)).magnitude() <= 0.9
                || (centre - Point::new(0.0, 0.2, 0.0)).magnitude() <= 0.9
                || (centre - Point::new(-4.0, 0.2, 0.0)).magnitude() <= 0.9
            {
                continue;
            }

            let material = match random() {
                0.0..0.33 => make_material_shareable(Lambertian::new(Colour::new(
                    random(),
                    random(),
                    random(),
                ))),
                0.33..0.66 => make_material_shareable(Metal::new(
                    random::<f64>() * Colour::new(random(), random(), random()),
                    random::<f64>(),
                )),
                0.66..0.90 => {
                    make_material_shareable(Light::new(Colour::new(random(), random(), random())))
                }
                _ => make_material_shareable(Dielectric::new(random::<f64>() / 2.0 + 0.75)),
            };

            world.add(create_world_object(
                centre.x,
                centre.y,
                centre.z,
                0.2,
                material.clone(),
            ));
        }
    }

    let material_ground =
        make_material_shareable(Lambertian::new(0.5 * Colour::new(1.0, 1.0, 1.0)));
    let material_left = make_material_shareable(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    let material_glass = make_material_shareable(Dielectric::new(1.5));
    let material_bubble = make_material_shareable(Dielectric::new(1.0 / 1.5));
    let material_right = make_material_shareable(Light::new(Colour::new(1.0, 0.9, 0.4)));
    let material_centre = make_material_shareable(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));

    //  The ground
    world.add(create_world_object(
        0.0,
        -1000.0,
        0.0,
        999.99,
        material_ground.clone(),
    ));

    // Leftmost big ball in the render
    world.add(create_world_object(
        -4.0,
        1.0,
        0.0,
        1.0,
        material_left.clone(),
    ));

    // Centre big ball in the render
    world.add(create_world_object(
        0.0,
        1.0,
        0.0,
        1.0,
        material_centre.clone(),
    ));

    // Rightmost big ball in the render
    world.add(create_world_object(
        4.0,
        1.0,
        0.0,
        0.9,
        material_bubble.clone(),
    ));
    world.add(create_world_object(
        4.0,
        1.0,
        0.0,
        1.0,
        material_glass.clone(),
    ));

    world.add(create_world_object(
        4.0,
        1.0,
        0.0,
        0.8,
        material_right.clone(),
    ));

    camera.render(Arc::new(Box::new(world)), 16);
}

fn make_material_shareable(
    material: impl Material + 'static,
) -> Arc<Box<dyn Material + Send + Sync>> {
    Arc::new(Box::new(material) as Box<dyn Material + Send + Sync>)
}

fn create_world_object(
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
    material: Arc<Box<dyn Material + Send + Sync>>,
) -> Arc<Box<dyn Hittable>> {
    Arc::new(Box::new(Sphere::new(
        Point::new(x, y, z),
        radius,
        material.clone(),
    )))
}
