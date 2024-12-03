use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    geometry::{transform::Transform, vector, Scalar, Vector},
    utils::approx::ApproxEq,
};

use super::Shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(Vector);

impl Point {
    const ORIGIN: Self = Self::new(0.0, 0.0);

    pub const fn new(x: Scalar, y: Scalar) -> Self {
        Self(vector!(x, y))
    }

    pub const fn as_vector(self) -> Vector {
        self.0
    }

    pub const fn vector_to(self, other: Point) -> Vector {
        other.0.sub(self.0)
    }

    pub fn distance(self, other: Point) -> Scalar {
        (other - self).norm()
    }

    pub const fn add(self, vector: Vector) -> Point {
        Self(self.0.add(vector))
    }

    pub const fn sub(self, vector: Vector) -> Point {
        Self(self.0.sub(vector))
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::ORIGIN
    }
}

impl ApproxEq for Point {
    type Epsilon = <Vector as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.approx_eq(&other.0, epsilon)
    }
}

impl Shape for Point {
    fn transform(&mut self, t: impl Transform) {
        *self = Self(t * self.0)
    }
}

impl From<Vector> for Point {
    fn from(value: Vector) -> Self {
        Self(value)
    }
}

impl From<Point> for Vector {
    fn from(value: Point) -> Self {
        value.0
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        self.add(rhs)
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        self.sub(rhs)
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        *self = *self - rhs;
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        rhs.vector_to(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::{scalar, transform::Translation},
        utils::approx::assert_approx_eq,
    };

    use super::*;

    #[test]
    fn as_vector() {
        let p = Point::new(1.0, 2.0);
        assert_approx_eq!(p.as_vector(), vector!(1.0, 2.0));
    }

    #[test]
    fn vector_to() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 5.0);

        assert_approx_eq!(p1.vector_to(p2), vector!(2.0, 3.0));
        assert_approx_eq!(p2 - p1, vector!(2.0, 3.0));
    }

    #[test]
    fn add() {
        let p = Point::new(1.0, 2.0);
        assert_approx_eq!(p + vector!(-2.0, 3.0), Point::new(-1.0, 5.0));
    }

    #[test]
    fn sub() {
        let p = Point::new(1.0, 2.0);
        assert_approx_eq!(p - vector!(-2.0, 3.0), Point::new(3.0, -1.0));
    }

    #[test]
    fn add_assign() {
        let mut p = Point::new(1.0, 2.0);
        p += vector!(-2.0, 3.0);
        assert_approx_eq!(p, Point::new(-1.0, 5.0));
    }

    #[test]
    fn sub_assign() {
        let mut p = Point::new(1.0, 2.0);
        p -= vector!(-2.0, 3.0);
        assert_approx_eq!(p, Point::new(3.0, -1.0));
    }

    #[test]
    fn transform() {
        let mut p = Point::new(1.0, 2.0);
        p.transform(Translation::from(vector!(-1.0, -2.0)));
        assert_approx_eq!(p, Point::ORIGIN);
    }

    #[test]
    fn distance() {
        let p1 = Point::new(1.0, 3.0);
        let p2 = Point::new(2.0, 4.0);
        assert_approx_eq!(p1.distance(p2), scalar::consts::SQRT_2);
    }
}
