use crate::vector::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }
}
