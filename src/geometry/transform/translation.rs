use std::ops::{Mul, MulAssign};

use crate::{geometry::Vector, utils::approx::ApproxEq};

use super::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Translation(Vector);

impl Translation {
    pub const IDENT: Self = Self::new(Vector::ZERO);

    pub const fn new(vector: Vector) -> Self {
        Self(vector)
    }

    pub const fn vector(self) -> Vector {
        self.0
    }

    pub const fn compose(self, other: Translation) -> Translation {
        Self::new(Vector::add(self.0, other.0))
    }

    pub const fn apply(self, vector: Vector) -> Vector {
        vector.add(self.0)
    }
}

impl From<Translation> for Vector {
    fn from(value: Translation) -> Self {
        value.vector()
    }
}

impl From<Vector> for Translation {
    fn from(value: Vector) -> Self {
        Self::new(value)
    }
}

impl Default for Translation {
    fn default() -> Self {
        Self::IDENT
    }
}

impl ApproxEq for Translation {
    type Epsilon = <Vector as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.approx_eq(&other.0, epsilon)
    }
}

impl Mul<Translation> for Translation {
    type Output = Translation;

    fn mul(self, rhs: Translation) -> Self::Output {
        self.compose(rhs)
    }
}

impl MulAssign<Translation> for Translation {
    fn mul_assign(&mut self, rhs: Translation) {
        *self = *self * rhs
    }
}

impl Mul<Vector> for Translation {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.apply(rhs)
    }
}

impl Transform for Translation {}

#[cfg(test)]
mod tests {
    use crate::{geometry::vector, utils::approx::assert_approx_eq};

    use super::*;

    #[test]
    fn translate() {
        let translation = Translation::from(vector!(-1.0, 2.0));
        assert_approx_eq!(translation * vector!(3.0, 4.0), vector!(2.0, 6.0));
    }

    #[test]
    fn compose() {
        let translation_1 = Translation::from(vector!(2.0, 3.0));
        let translation_2 = Translation::from(vector!(-3.0, 4.0));
        assert_approx_eq!(
            translation_1 * translation_2,
            Translation::from(vector!(-1.0, 7.0))
        );
    }

    #[test]
    fn compose_assign() {
        let mut translation = Translation::from(vector!(2.0, 3.0));
        translation *= Translation::from(vector!(-3.0, 4.0));
        assert_approx_eq!(translation, Translation::from(vector!(-1.0, 7.0)));
    }
}
