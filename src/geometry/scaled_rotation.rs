use std::ops::Mul;

use crate::utils::approx::ApproxEq;

use super::{vector, Scalar, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaledRotation(Vector);

impl ScaledRotation {
    pub const IDENT: Self = Self::new(Vector::X);

    pub const fn new(complex: Vector) -> Self {
        Self(complex)
    }

    pub fn from_parts(scale: Scalar, rotation: Scalar) -> Self {
        Self::new(vector!(scale * rotation.cos(), scale * rotation.sin()))
    }

    pub const fn re(self) -> Scalar {
        self.0.x
    }

    pub const fn im(self) -> Scalar {
        self.0.y
    }

    pub fn scale(self) -> Scalar {
        vector!(self.re(), self.im()).norm()
    }

    pub fn rotation(self) -> Scalar {
        Scalar::atan2(self.im(), self.re())
    }

    pub const fn compose(self, other: ScaledRotation) -> ScaledRotation {
        Self::new(self.0.complex_product(other.0))
    }

    pub const fn apply(self, vector: Vector) -> Vector {
        self.0.complex_product(vector)
    }
}

impl Default for ScaledRotation {
    fn default() -> Self {
        Self::IDENT
    }
}

impl ApproxEq for ScaledRotation {
    type Epsilon = <Vector as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.approx_eq(&other.0, epsilon)
    }
}

impl Mul<ScaledRotation> for ScaledRotation {
    type Output = ScaledRotation;

    fn mul(self, rhs: ScaledRotation) -> Self::Output {
        self.compose(rhs)
    }
}

impl Mul<Vector> for ScaledRotation {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.apply(rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::scalar, utils::approx::assert_approx_eq};

    use super::*;

    #[test]
    fn scale() {
        let scale = ScaledRotation::from_parts(2.0, 0.0);
        assert_approx_eq!(scale * Vector::X, 2.0 * Vector::X)
    }

    #[test]
    fn rotate() {
        let rot = ScaledRotation::from_parts(1.0, 0.25 * scalar::consts::TAU);
        assert_approx_eq!(rot * Vector::X, Vector::Y)
    }

    #[test]
    fn scale_rot() {
        let scale_rot = ScaledRotation::from_parts(2.0, 0.25 * scalar::consts::TAU);
        assert_approx_eq!(scale_rot * Vector::Y, -2.0 * Vector::X)
    }

    #[test]
    fn compose() {
        let t1 = ScaledRotation::from_parts(2.0, 0.5 * scalar::consts::TAU);
        let t2 = ScaledRotation::from_parts(3.0, -0.25 * scalar::consts::TAU);

        assert_approx_eq!(
            t1 * t2,
            ScaledRotation::from_parts(6.0, 0.25 * scalar::consts::TAU)
        );
    }

    #[test]
    fn extract_parts() {
        let t = ScaledRotation::from_parts(2.0, 0.25 * scalar::consts::TAU);

        assert_approx_eq!(t.scale(), 2.0);
        assert_approx_eq!(t.rotation(), 0.25 * scalar::consts::TAU);
    }
}
