use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::approx::{approx_eq, ApproxEq};

use super::Scalar;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vector {
    pub const ZERO: Self = vector!(0.0, 0.0);
    pub const X: Self = vector!(1.0, 0.0);
    pub const Y: Self = vector!(0.0, 1.0);

    #[inline]
    pub const fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn scale(self, scalar: Scalar) -> Self {
        vector!(self.x * scalar, self.y * scalar)
    }

    #[inline]
    pub const fn add(self, other: Vector) -> Self {
        vector!(self.x + other.x, self.y + other.y)
    }

    #[inline]
    pub const fn sub(self, other: Vector) -> Self {
        vector!(self.x - other.x, self.y - other.y)
    }

    #[inline]
    pub const fn perp(self) -> Vector {
        vector!(-self.y, self.x)
    }

    #[inline]
    pub const fn dot(self, other: Vector) -> Scalar {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub const fn perp_dot(self, other: Vector) -> Scalar {
        self.perp().dot(other)
    }

    #[inline]
    pub const fn norm_sq(self) -> Scalar {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn norm(self) -> Scalar {
        self.norm_sq().sqrt()
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::ZERO
    }
}

impl ApproxEq for Vector {
    type Tolerance = <Scalar as ApproxEq>::Tolerance;

    fn approx_eq(&self, other: &Self, tolerance: Self::Tolerance) -> bool {
        approx_eq!(self.x, other.x, tolerance) && approx_eq!(self.y, other.y, tolerance)
    }
}

impl Mul<Scalar> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Scalar) -> Self::Output {
        self.scale(rhs)
    }
}

impl Mul<Vector> for Scalar {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs.scale(self)
    }
}

impl Div<Scalar> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: Scalar) -> Self::Output {
        self.scale(1.0 / rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Self::Output {
        self.scale(-1.0)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        self.sub(rhs)
    }
}

macro_rules! vector {
    ($x:expr, $y:expr) => {
        $crate::geometry::Vector::new($x, $y)
    };
}

pub(crate) use vector;

#[cfg(test)]
mod tests {
    use crate::utils::approx::{assert_approx_eq, assert_approx_ne};

    use super::*;

    #[test]
    fn scale() {
        let vec = vector!(1.0, 2.0);
        assert_eq!(vec * 2.0, vector!(2.0, 4.0));
        assert_eq!(2.0 * vec, vector!(2.0, 4.0));
        assert_eq!(vec / 2.0, vector!(0.5, 1.0));
        assert_eq!(vec.scale(2.0), vector!(2.0, 4.0));
        assert_eq!(-vec, vector!(-1.0, -2.0));
    }

    #[test]
    fn add() {
        assert_eq!(vector!(4.0, 12.0) + vector!(2.0, -3.0), vector!(6.0, 9.0))
    }

    #[test]
    fn sub() {
        assert_eq!(vector!(4.0, 12.0) - vector!(2.0, -3.0), vector!(2.0, 15.0))
    }

    #[test]
    fn perp() {
        assert_eq!(vector!(3.0, 5.0).perp(), vector!(-5.0, 3.0))
    }

    #[test]
    fn dot() {
        assert_eq!(Vector::dot(vector!(1.0, -2.0), vector!(3.0, 4.0)), -5.0)
    }

    #[test]
    fn perp_dot() {
        assert_eq!(
            Vector::perp_dot(vector!(1.0, -2.0), vector!(3.0, 4.0)),
            10.0
        )
    }

    #[test]
    fn norm_sq() {
        assert_eq!(Vector::ZERO.norm_sq(), 0.0);
        assert_eq!(vector!(3.0, 4.0).norm_sq(), 25.0);
    }

    #[test]
    fn norm() {
        assert_eq!(Vector::ZERO.norm(), 0.0);
        assert_eq!(vector!(3.0, 4.0).norm(), 5.0);
    }

    #[test]
    fn approx_eq() {
        assert_approx_eq!(Vector::X, Vector::X + Vector::X * Scalar::EPSILON);
        assert_approx_ne!(Vector::X, Vector::X + Vector::X * Scalar::EPSILON * 2.0);
    }
}
