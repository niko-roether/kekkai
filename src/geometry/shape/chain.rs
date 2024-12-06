use std::{
    iter::{self, Copied, Peekable},
    ops::{Mul, MulAssign},
    slice,
};

use crate::{
    geometry::{transform::Similarity, Scalar},
    utils::approx::ApproxEq,
};

use super::{Point, Segment};

#[derive(Debug, Clone, PartialEq)]
pub struct Chain {
    pub vertices: Vec<Point>,
}

impl Chain {
    pub const fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }

    pub fn vertices(&self) -> Vertices {
        self.vertices.iter().copied()
    }

    pub fn vertices_mut(&mut self) -> VerticesMut {
        self.vertices.iter_mut()
    }

    pub fn segments(&self) -> Segments {
        Segments {
            vertices: self.vertices.iter().peekable(),
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_segments(&self) -> usize {
        self.vertices.len().saturating_sub(1)
    }

    pub fn distance_to_point(&self, point: Point) -> Scalar {
        self.segments()
            .map(|s| s.distance_to_point(point))
            .reduce(Scalar::min)
            .unwrap_or(Scalar::INFINITY)
    }

    pub fn transform(&mut self, t: &Similarity) {
        self.vertices_mut().for_each(|v| *v *= t);
    }
}

impl ApproxEq for Chain {
    type Epsilon = <Scalar as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.num_vertices() != other.num_vertices() {
            return false;
        }
        for (v1, v2) in iter::zip(self.vertices(), other.vertices()) {
            if v1.approx_ne(&v2, epsilon) {
                return false;
            }
        }
        true
    }
}

impl Mul<Chain> for &Similarity {
    type Output = Chain;

    fn mul(self, mut rhs: Chain) -> Self::Output {
        rhs.transform(self);
        rhs
    }
}

impl MulAssign<&Similarity> for Chain {
    fn mul_assign(&mut self, rhs: &Similarity) {
        self.transform(rhs);
    }
}

impl From<Point> for Chain {
    fn from(value: Point) -> Self {
        Self::new(vec![value])
    }
}

impl From<Segment> for Chain {
    fn from(value: Segment) -> Self {
        Self::new(vec![value.start, value.end])
    }
}

impl FromIterator<Point> for Chain {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

pub type Vertices<'a> = Copied<slice::Iter<'a, Point>>;
pub type VerticesMut<'a> = slice::IterMut<'a, Point>;

pub struct Segments<'a> {
    vertices: Peekable<slice::Iter<'a, Point>>,
}

impl<'a> Iterator for Segments<'a> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        let start = *self.vertices.next()?;
        let end = **self.vertices.peek()?;
        Some(Segment::new(start, end))
    }
}

impl<'a> ExactSizeIterator for Segments<'a> {
    fn len(&self) -> usize {
        self.vertices.len().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::{scalar, transform::Translation, vector},
        utils::approx::assert_approx_eq,
    };

    use super::*;

    #[test]
    fn vertices() {
        let chain = Chain::new(vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 2.0),
            Point::new(4.0, 3.0),
        ]);

        let mut vertices = chain.vertices();
        assert_eq!(vertices.next(), Some(Point::new(1.0, 2.0)));
        assert_eq!(vertices.next(), Some(Point::new(3.0, 2.0)));
        assert_eq!(vertices.next(), Some(Point::new(4.0, 3.0)));
        assert_eq!(vertices.next(), None);
    }

    #[test]
    fn vertices_mut() {
        let mut chain = Chain::new(vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 2.0),
            Point::new(4.0, 3.0),
        ]);

        let mut vertices = chain.vertices_mut();
        assert_eq!(vertices.next(), Some(&mut Point::new(1.0, 2.0)));
        assert_eq!(vertices.next(), Some(&mut Point::new(3.0, 2.0)));
        assert_eq!(vertices.next(), Some(&mut Point::new(4.0, 3.0)));
        assert_eq!(vertices.next(), None);
    }

    #[test]
    fn segments() {
        let chain = Chain::new(vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 2.0),
            Point::new(4.0, 3.0),
        ]);

        let mut segments = chain.segments();
        assert_eq!(
            segments.next(),
            Some(Segment::new(Point::new(1.0, 2.0), Point::new(3.0, 2.0)))
        );
        assert_eq!(
            segments.next(),
            Some(Segment::new(Point::new(3.0, 2.0), Point::new(4.0, 3.0)))
        );
        assert_eq!(segments.next(), None);
    }

    #[test]
    fn lengths() {
        let chain = Chain::new(vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 2.0),
            Point::new(4.0, 3.0),
        ]);

        assert_eq!(chain.num_vertices(), 3);
        assert_eq!(chain.num_segments(), 2);
    }

    #[test]
    fn distance_to_point() {
        let chain = Chain::new(vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 2.0),
            Point::new(4.0, 3.0),
        ]);

        assert_approx_eq!(
            chain.distance_to_point(Point::new(3.0, 3.0)),
            scalar::consts::FRAC_1_SQRT_2
        );
    }

    #[test]
    fn transform() {
        let mut chain = Chain::new(vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 2.0),
            Point::new(4.0, 3.0),
        ]);

        chain *= &Translation::from(vector!(1.0, 0.0)).into();
        assert_approx_eq!(
            chain,
            Chain::new(vec![
                Point::new(2.0, 2.0),
                Point::new(4.0, 2.0),
                Point::new(5.0, 3.0)
            ])
        )
    }
}
