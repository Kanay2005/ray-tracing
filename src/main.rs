use std::sync::Arc;

use cgmath::InnerSpace;
use rand::random;
use ray_tracing::Colour;
use ray_tracing::camera::Camera;
use ray_tracing::hittable_list::*;
use ray_tracing::material::Dielectric;
use ray_tracing::material::Lambertian;
use ray_tracing::material::Light;
use ray_tracing::material::Material;
use ray_tracing::material::Metal;
use ray_tracing::ray::*;
use ray_tracing::sphere::*;

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        640,
        5,
        50,
        20.0,
        Vector::new(13.0, 2.0, 3.0),
        Vector::new(0.0, 0.0, 0.0),
        0.6,
        10.0,
    );

    let mut world = HittableList::new();
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
                0.0..0.33 => Arc::new(Box::new(Lambertian::new(Colour::new(
                    random(),
                    random(),
                    random(),
                ))) as Box<dyn Material + Send + Sync>),
                0.33..0.66 => Arc::new(Box::new(Metal::new(
                    random::<f64>() * Colour::new(random(), random(), random()),
                    random::<f64>(),
                )) as Box<dyn Material + Send + Sync>),
                0.66..0.90 => Arc::new(Box::new(Light::new(Colour::new(
                    random(),
                    random(),
                    random(),
                ))) as Box<dyn Material + Send + Sync>),
                _ => Arc::new(Box::new(Dielectric::new(random::<f64>() / 2.0 + 0.75))
                    as Box<dyn Material + Send + Sync>),
            };

            world.add(Arc::new(Box::new(Sphere::new(
                centre,
                0.2,
                material.clone(),
            ))));
        }
    }

    let material_ground = Arc::new(Box::new(Lambertian::new(0.5 * Colour::new(1.0, 1.0, 1.0)))
        as Box<dyn Material + Send + Sync>);
    let material_left =
        Arc::new(Box::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)))
            as Box<dyn Material + Send + Sync>);
    // let material_light = Arc::new(
    //     Box::new(Light::new(Colour::new(1.0, 1.0, 1.0))) as Box<dyn Material + Send + Sync>
    // );
    let material_glass =
        Arc::new(Box::new(Dielectric::new(1.5)) as Box<dyn Material + Send + Sync>);
    let material_bubble =
        Arc::new(Box::new(Dielectric::new(1.0 / 1.5)) as Box<dyn Material + Send + Sync>);
    let material_right = Arc::new(
        Box::new(Light::new(Colour::new(1.0, 0.9, 0.4))) as Box<dyn Material + Send + Sync>
    );
    let material_centre =
        Arc::new(Box::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0))
            as Box<dyn Material + Send + Sync>);

    // Arc::new(Box::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0)) as Box<dyn Material>);

    // World
    // let mut world = HittableList::new();
    world.add(Arc::new(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        999.99,
        material_ground.clone(),
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_left.clone(),
    ))));
    // world.add(Arc::new(Box::new(Sphere::new(
    //     Point::new(0.0, 1000.0, 500.0),
    //     500.0,
    //     material_light.clone(),
    // ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_centre.clone(),
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_glass.clone(),
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        0.9,
        material_bubble.clone(),
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        0.8,
        material_right.clone(),
    ))));

    camera.render(Arc::new(Box::new(world)), 16);
}
