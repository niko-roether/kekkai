use crate::geometry::{transform::Similarity, Scalar};

use super::Point;

pub struct Circle {
    pub center: Point,
    pub radius: Scalar,
}

impl Circle {
    pub const fn new(center: Point, radius: Scalar) -> Self {
        Self { center, radius }
    }

    pub const fn centered(radius: Scalar) -> Self {
        Self::new(Point::ORIGIN, radius)
    }

    pub fn signed_distance(&self, point: Point) -> Scalar {
        self.center.distance(point) - self.radius
    }

    pub fn transform(&mut self, t: &Similarity) {
        self.center *= t;
        self.radius *= t.scale();
    }
}
