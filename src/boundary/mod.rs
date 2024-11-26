use nalgebra::Vector2;

mod ellipse;
mod polygon;

pub use ellipse::Ellipse;
pub use polygon::Polygon;

#[derive(Debug, Clone)]
pub enum Boundary {
    Ellipse(Ellipse),
    Polygon(Polygon),
}

impl Boundary {
    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        match self {
            Self::Ellipse(ellipse) => ellipse.signed_distance(pos),
            Self::Polygon(polygon) => polygon.signed_distance(pos),
        }
    }
}

impl From<Ellipse> for Boundary {
    fn from(value: Ellipse) -> Self {
        Self::Ellipse(value)
    }
}

impl From<Polygon> for Boundary {
    fn from(value: Polygon) -> Self {
        Self::Polygon(value)
    }
}
