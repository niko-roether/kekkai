use nalgebra::Vector2;

mod circle;
mod polygon;

/// A geometric shape, considered up to isometry.
pub trait Shape: Clone {
    type ConvexHull: ConvexShape;

    fn bounding_radius(&self) -> f32;

    fn contains(&self, point: Vector2<f32>) -> bool;

    fn convex_hull(&self) -> Self::ConvexHull;

    fn scale(&mut self, factor: f32);
}

/// A [`Shape`] that is known to be convex at compile time.
pub trait ConvexShape: Shape<ConvexHull = Self> {}
