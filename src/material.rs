use rand::{random, random_range};

use crate::{
    Colour,
    hittable::HitRecord,
    ray::Ray,
    vector::{Vector, dot},
};

pub struct RayRecord {
    pub colour: Colour,
    pub ray: Option<Ray>,
}

impl RayRecord {
    fn new(colour: Colour, ray: Option<Ray>) -> Self {
        RayRecord { colour, ray }
    }
}

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> RayRecord;
}

pub struct Lambertian {
    albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> RayRecord {
        let mut direction = rec.normal + random_unit_vector();
        if near_zero(direction) {
            direction = rec.normal;
        }

        RayRecord::new(self.albedo, Some(Ray::new(rec.point, direction)))
    }
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Lambertian { albedo }
    }
}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> RayRecord {
        // might have to change it so that the ones that are pointed backwards are not scattered.
        let reflected =
            reflect(r_in.direction, rec.normal).normalize() + (random_unit_vector() * self.fuzz);

        RayRecord::new(self.albedo, Some(Ray::new(rec.point, reflected)))
    }
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> RayRecord {
        let ri = match rec.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };
        let unit_direction = r_in.direction.normalize();

        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = match ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > random() {
            true => reflect(unit_direction, rec.normal),
            false => refract(unit_direction, rec.normal, ri),
        };

        RayRecord::new(
            Colour::new(1.0, 1.0, 1.0),
            Some(Ray::new(rec.point, direction)),
        )
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

pub struct Light {
    albedo: Colour,
}

impl Material for Light {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> RayRecord {
        RayRecord::new(self.albedo, None)
    }
}

impl Light {
    pub fn new(albedo: Colour) -> Self {
        Light { albedo }
    }
}

fn random_unit_vector() -> Vector {
    let random_vec = Vector::new(
        random_range(-1.0..=1.0),
        random_range(-1.0..=1.0),
        random_range(-1.0..=1.0),
    );
    random_vec.normalize()
}

fn near_zero(vec: Vector) -> bool {
    let min = 1e-8;
    vec.x.abs() < min && vec.y.abs() < min && vec.z.abs() < min
}

fn reflect(v: Vector, n: Vector) -> Vector {
    v - n * 2.0 * dot(v, n)
}

fn refract(v: Vector, n: Vector, etai_over_etat: f64) -> Vector {
    let cos_theta = dot(-v, n).min(1.0);
    let r_out_perp = (v + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -(1.0 - r_out_perp.magnitude2()).abs().sqrt();
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
