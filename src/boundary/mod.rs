use nalgebra::Vector2;

mod circle;
mod composite;
mod ellipse;
mod polygon;
mod rect;

pub use circle::Circle;
pub use composite::Composite;
pub use ellipse::Ellipse;
pub use polygon::Polygon;

#[derive(Debug, Clone)]
pub enum Boundary<'a> {
    Circle(Circle),
    Ellipse(Ellipse),
    Polygon(Polygon),
    Composite(Composite<'a>),
}

impl<'a> Boundary<'a> {
    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        match self {
            Self::Circle(circle) => circle.signed_distance(pos),
            Self::Ellipse(ellipse) => ellipse.signed_distance(pos),
            Self::Polygon(polygon) => polygon.signed_distance(pos),
            Self::Composite(composite) => composite.signed_distance(pos),
        }
    }
}

impl From<Circle> for Boundary<'static> {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}

impl From<Ellipse> for Boundary<'static> {
    fn from(value: Ellipse) -> Self {
        Self::Ellipse(value)
    }
}

impl From<Polygon> for Boundary<'static> {
    fn from(value: Polygon) -> Self {
        Self::Polygon(value)
    }
}

impl<'a> From<Composite<'a>> for Boundary<'a> {
    fn from(value: Composite<'a>) -> Self {
        Self::Composite(value)
    }
}
