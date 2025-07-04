use std::{
    cmp::max,
    f64::INFINITY,
    sync::{Arc, Mutex},
    thread,
};

use cgmath::{ElementWise, InnerSpace};
use image::{ImageBuffer, Rgb};

use crate::{
    Colour, get_colour_from_pixel,
    hittable::Hittable,
    interval::Interval,
    ray::{Ray, Vector},
};
use rand::{Rng, random_range};

pub struct Camera {
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    defocus_angle: f64,
    image_height: i32,
    pixel_samples_scale: f64,
    centre: Vector,
    pixel00_loc: Vector,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    defocus_disk_u: Vector,
    defocus_disk_v: Vector,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Vector,
        lookat: Vector,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = max((f64::from(image_width) / aspect_ratio) as i32, 1);
        let centre = lookfrom;

        let pixel_samples_scale = 1.0 / f64::from(samples_per_pixel);

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));

        let vup = Vector::new(0.0, 1.0, 0.0);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // Edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Pixel distances
        let pixel_delta_u = viewport_u / f64::from(image_width);
        let pixel_delta_v = viewport_v / f64::from(image_height);

        // Useful Vectors
        let viewport_upper_left = centre - (focus_dist * w) - 0.5 * (viewport_u + viewport_v);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            image_height,
            pixel_samples_scale,
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: Arc<Box<dyn Hittable + Sync + Send>>, num_threads: i32) {
        let mut pixels = vec![
            vec![Colour::new(0.0, 0.0, 0.0); self.image_width as usize];
            self.image_height as usize
        ];

        let mut chunks = Vec::with_capacity(num_threads as usize);
        let total_done = Arc::new(Mutex::new(0));
        thread::scope(|s| {
            let chunk_size = (self.image_height as f32 / num_threads as f32).ceil() as i32;
            for chunk in pixels.chunks_mut(chunk_size as usize) {
                chunks.push(Arc::new(Mutex::new(chunk)));
            }
            for cur_thread in 0..num_threads {
                let shared_world = world.clone();
                let cur_chunk = chunks[cur_thread as usize].clone();
                let cur_total = total_done.clone();
                s.spawn(move || {
                    let mut chunk = cur_chunk.lock().unwrap();
                    for j in
                        (cur_thread * chunk_size)..(cur_thread * chunk_size + chunk.len() as i32)
                    {
                        for i in 0..self.image_width {
                            for _ in 0..self.samples_per_pixel {
                                let cur_colour = self.ray_colour(
                                    &self.get_ray(i, j),
                                    shared_world.clone(),
                                    self.max_depth,
                                );
                                chunk[(j % chunk_size) as usize][i as usize] += cur_colour;
                            }
                        }
                        let mut total = cur_total.lock().unwrap();
                        *total += 1;
                        println!(
                            "Line {} Complete, {}/{} Total",
                            j + 1,
                            *total,
                            self.image_height
                        );
                        drop(total);
                    }
                });
            }
        });

        let mut img_buf = ImageBuffer::new(self.image_width as u32, self.image_height as u32);
        for (x, y, pixel_out) in img_buf.enumerate_pixels_mut() {
            let (r, g, b) =
                get_colour_from_pixel(pixels[y as usize][x as usize] * self.pixel_samples_scale);
            *pixel_out = Rgb([r, g, b]);
        }
        let _ = img_buf.save("image.jpg");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let mut rng = rand::rng();
        let offset = Vector::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0);

        let pixel = self.pixel00_loc
            + (f64::from(i) + offset.x) * self.pixel_delta_u
            + (f64::from(j) + offset.y) * self.pixel_delta_v;

        let origin = match self.defocus_angle <= 0.0 {
            true => self.centre,
            false => self.defocus_disk_sample(),
        };
        let direction = pixel - origin;

        Ray::new(origin, direction)
    }

    fn ray_colour(
        &self,
        ray: &Ray,
        world: Arc<Box<dyn Hittable + Sync + Send>>,
        depth: i32,
    ) -> Colour {
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            let ray_record = rec.material.scatter(&ray, &rec);
            return match ray_record.ray {
                Some(scattered) => ray_record.colour.mul_element_wise(self.ray_colour(
                    &scattered,
                    world,
                    depth - 1,
                )),
                None => ray_record.colour,
            };
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
        // 0.25 * Colour::new(111.0 / 255.0, 144.0 / 255.0, 168.0 / 255.0)
    }

    fn defocus_disk_sample(&self) -> Vector {
        let p = random_in_unit_disk();
        self.centre + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

fn random_in_unit_disk() -> Vector {
    loop {
        let p = Vector::new(random_range(-1.0..=1.0), random_range(-1.0..=1.0), 0.0);
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}
