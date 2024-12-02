use std::ops::{Mul, MulAssign};

use crate::{
    geometry::{Scalar, Vector},
    utils::approx::{approx_eq, ApproxEq},
};

use super::{ScaledRotation, Translation};

#[derive(Debug, Clone, PartialEq)]
pub struct Similarity {
    scaled_rotation: ScaledRotation,
    translation: Translation,
}

impl Similarity {
    pub const IDENT: Self = Self::new(ScaledRotation::IDENT, Translation::IDENT);

    pub const fn new(scaled_rotation: ScaledRotation, translation: Translation) -> Self {
        Self {
            scaled_rotation,
            translation,
        }
    }

    pub fn from_parts(scale: Scalar, rotation: Scalar, translation_vector: Vector) -> Self {
        Self::new(
            ScaledRotation::from_parts(scale, rotation),
            Translation::from(translation_vector),
        )
    }

    pub const fn scaled_rotation(&self) -> ScaledRotation {
        self.scaled_rotation
    }

    pub fn scale(&self) -> Scalar {
        self.scaled_rotation.scale()
    }

    pub fn rotation(&self) -> Scalar {
        self.scaled_rotation.rotation()
    }

    pub const fn translation(&self) -> Translation {
        self.translation
    }

    pub const fn translation_vector(&self) -> Vector {
        self.translation.vector()
    }

    pub const fn compose(&self, other: &Self) -> Self {
        Self::new(
            self.scaled_rotation.compose(other.scaled_rotation),
            self.translation.compose(other.translation),
        )
    }

    pub const fn apply(&self, vector: Vector) -> Vector {
        self.translation.apply(self.scaled_rotation.apply(vector))
    }
}

impl Default for Similarity {
    fn default() -> Self {
        Self::IDENT
    }
}

impl ApproxEq for Similarity {
    type Epsilon = <Scalar as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        approx_eq!(self.scaled_rotation, other.scaled_rotation, epsilon)
            && approx_eq!(self.translation, other.translation, epsilon)
    }
}

impl Mul<&Similarity> for &Similarity {
    type Output = Similarity;

    fn mul(self, rhs: &Similarity) -> Self::Output {
        self.compose(rhs)
    }
}

impl MulAssign<&Similarity> for Similarity {
    fn mul_assign(&mut self, rhs: &Similarity) {
        *self = &*self * rhs
    }
}

impl Mul<Vector> for &Similarity {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.apply(rhs)
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
    fn apply() {
        let sim = Similarity::from_parts(3.0, 0.25 * scalar::consts::TAU, vector!(2.0, 3.0));

        assert_approx_eq!(&sim * Vector::Y, vector!(-1.0, 3.0), 2.0 * Scalar::EPSILON);
    }

    #[test]
    fn compose() {
        let sim_1 = Similarity::from_parts(3.0, 0.25 * scalar::consts::TAU, vector!(2.0, 3.0));
        let sim_2 = Similarity::from_parts(2.0, -0.5 * scalar::consts::TAU, vector!(-1.0, 1.0));

        assert_approx_eq!(
            &sim_1 * &sim_2,
            Similarity::from_parts(6.0, -0.25 * scalar::consts::TAU, vector!(1.0, 4.0))
        );
    }

    #[test]
    fn compose_assign() {
        let mut sim = Similarity::from_parts(3.0, 0.25 * scalar::consts::TAU, vector!(2.0, 3.0));
        sim *= &Similarity::from_parts(2.0, -0.5 * scalar::consts::TAU, vector!(-1.0, 1.0));

        assert_approx_eq!(
            sim,
            Similarity::from_parts(6.0, -0.25 * scalar::consts::TAU, vector!(1.0, 4.0))
        );
    }
}
