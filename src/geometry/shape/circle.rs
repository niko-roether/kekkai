use std::ops::{Mul, MulAssign};

use crate::{
    geometry::{transform::Similarity, Scalar},
    utils::approx::ApproxEq,
};

use super::Point;

#[derive(Debug, Clone, PartialEq)]
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

impl ApproxEq for Circle {
    type Epsilon = <Scalar as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.center.approx_eq(&other.center, epsilon)
            && self.radius.approx_eq(&other.radius, epsilon)
    }
}

impl Mul<Circle> for &Similarity {
    type Output = Circle;

    fn mul(self, mut rhs: Circle) -> Self::Output {
        rhs.transform(self);
        rhs
    }
}

impl MulAssign<&Similarity> for Circle {
    fn mul_assign(&mut self, rhs: &Similarity) {
        self.transform(rhs);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::{scalar, vector},
        utils::approx::assert_approx_eq,
    };

    use super::*;

    #[test]
    fn signed_distance_positive() {
        let circle = Circle::new(Point::new(1.0, 1.0), 1.0);
        assert_approx_eq!(
            circle.signed_distance(Point::ORIGIN),
            scalar::consts::SQRT_2 - 1.0
        )
    }

    #[test]
    fn signed_distance_zero() {
        let circle = Circle::new(Point::new(1.0, 1.0), 1.0);
        assert_approx_eq!(circle.signed_distance(Point::new(1.0, 0.0)), 0.0)
    }

    #[test]
    fn signed_distance_negative() {
        let circle = Circle::new(Point::new(1.0, 1.0), 1.0);
        assert_approx_eq!(circle.signed_distance(Point::new(1.0, 1.0)), -1.0)
    }

    #[test]
    fn transform() {
        let mut circle = Circle::new(Point::new(1.0, 2.0), 1.0);
        circle *= &Similarity::from_parts(2.0, scalar::consts::TAU / 4.0, vector!(2.0, -1.0));
        assert_approx_eq!(circle, Circle::new(Point::new(-2.0, 1.0), 2.0), 2.0);
    }
}
