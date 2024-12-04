mod dist;
mod point;
mod segment;

use std::ops::{Mul, MulAssign};

use super::{transform::Transform, Scalar};

pub use point::*;
pub use segment::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Point(Point),
    Segment(Segment),
}

impl Shape {
    pub fn transform(&mut self, t: impl Transform) {
        match self {
            Self::Point(point) => point.transform(t),
            Self::Segment(segment) => segment.transform(t),
        }
    }

    pub fn signed_distance(&self, other: &Self) -> Scalar {
        match (self, other) {
            (Self::Point(a), Self::Point(b)) => dist::point_to_point(*a, *b),
            (Self::Segment(s1), Self::Segment(s2)) => dist::segment_to_segment(s1, s2),
            (Self::Point(p), Self::Segment(segment)) | (Self::Segment(segment), Self::Point(p)) => {
                dist::point_to_segment(*p, segment)
            }
        }
    }
}

impl<T: Transform> MulAssign<T> for &mut Shape {
    fn mul_assign(&mut self, rhs: T) {
        self.transform(rhs);
    }
}

impl<T: Transform> Mul<T> for Shape {
    type Output = Shape;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.transform(rhs);
        self
    }
}

impl From<Point> for Shape {
    fn from(value: Point) -> Self {
        Shape::Point(value)
    }
}

impl From<Segment> for Shape {
    fn from(value: Segment) -> Self {
        Shape::Segment(value)
    }
}
