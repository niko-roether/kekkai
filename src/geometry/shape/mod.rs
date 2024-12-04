mod circle;
mod dist;
mod point;
mod segment;

use std::ops::{Mul, MulAssign};

use super::{transform::Similarity, Scalar};

pub use circle::*;
pub use point::*;
pub use segment::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Point(Point),
    Segment(Segment),
}

impl Shape {
    pub fn transform(&mut self, t: &Similarity) {
        match self {
            Self::Point(point) => *point *= t,
            Self::Segment(segment) => *segment *= t,
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

impl MulAssign<&Similarity> for &mut Shape {
    fn mul_assign(&mut self, rhs: &Similarity) {
        self.transform(rhs);
    }
}

impl Mul<&Similarity> for Shape {
    type Output = Shape;

    fn mul(mut self, rhs: &Similarity) -> Self::Output {
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
