use nalgebra::Vector2;

use super::{ConvexShape, Shape};

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Shape for Circle {
    type ConvexHull = Self;

    fn bounding_radius(&self) -> f32 {
        self.radius
    }

    fn convex_hull(&self) -> Self::ConvexHull {
        self.clone()
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        point.norm_squared() <= self.radius.powi(2)
    }

    fn scale(&mut self, factor: f32) {
        self.radius *= factor
    }
}

impl ConvexShape for Circle {}
