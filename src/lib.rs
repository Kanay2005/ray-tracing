use cgmath::Vector3;
use interval::Interval;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub type Colour = Vector3<f64>;

fn linear_to_gamma(x: f64) -> f64 {
    match x > 0.0 {
        true => x.sqrt(),
        false => 0.0,
    }
}

pub fn get_colour_from_pixel(pixel: Colour) -> (u8, u8, u8) {
    let interval = Interval::new(0.0, 0.999);

    let r = linear_to_gamma(pixel.x);
    let g = linear_to_gamma(pixel.y);
    let b = linear_to_gamma(pixel.z);

    let r_byte = (255.99 * interval.clamp(r)) as u8;
    let g_byte = (255.99 * interval.clamp(g)) as u8;
    let b_byte = (255.99 * interval.clamp(b)) as u8;
    (r_byte, g_byte, b_byte)
}
