#![allow(unused_macros, dead_code)]

use std::fmt::Display;

pub(crate) trait Epsilon: Copy + Display {
    fn default() -> Self;
}

impl Epsilon for f32 {
    fn default() -> Self {
        f32::EPSILON
    }
}

impl Epsilon for f64 {
    fn default() -> Self {
        f64::EPSILON
    }
}

pub(crate) trait ApproxEq {
    type Epsilon: Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool;

    fn approx_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !self.approx_eq(other, epsilon)
    }
}

impl ApproxEq for f32 {
    type Epsilon = f32;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (other - self).abs() <= epsilon
    }
}

impl ApproxEq for f64 {
    type Epsilon = f64;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (other - self).abs() <= epsilon
    }
}

impl<T: ApproxEq> ApproxEq for &T {
    type Epsilon = T::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (**self).approx_eq(&**other, epsilon)
    }
}

macro_rules! approx_eq {
    ($val1:expr, $val2:expr, $epsilon:expr) => {
        $crate::utils::approx::ApproxEq::approx_eq(&$val1, &$val2, $epsilon)
    };
    ($val1:expr, $val2:expr) => {
        $crate::utils::approx::ApproxEq::approx_eq(
            &$val1,
            &$val2,
            $crate::utils::approx::Epsilon::default(),
        )
    };
}

#[allow(unused)]
pub(crate) use approx_eq;

macro_rules! approx_ne {
    ($val1:expr, $val2:expr, $epsilon:expr) => {
        $crate::utils::approx::ApproxEq::approx_ne(&$val1, &$val2, $epsilon)
    };
    ($val1:expr, $val2:expr) => {
        $crate::utils::approx::ApproxEq::approx_ne(
            &$val1,
            &$val2,
            $crate::utils::approx::Epsilon::default(),
        )
    };
}

#[allow(unused)]
pub(crate) use approx_ne;

macro_rules! assert_approx_eq {
    ($val1:expr, $val2:expr, $epsilon:expr) => {{
        let epsilon = $epsilon;
        if $crate::utils::approx::approx_ne!($val1, $val2, epsilon) {
            panic!("Assertion failed: {:?} ≈ {:?} (±{})", $val1, $val2, epsilon)
        }
    }};
    ($val1:expr, $val2:expr) => {
        $crate::utils::approx::assert_approx_eq!(
            $val1,
            $val2,
            $crate::utils::approx::Epsilon::default()
        )
    };
}

#[allow(unused)]
pub(crate) use assert_approx_eq;

macro_rules! assert_approx_ne {
    ($val1:expr, $val2:expr, $epsilon:expr) => {{
        let epsilon = $epsilon;
        if $crate::utils::approx::approx_eq!($val1, $val2, epsilon) {
            panic!("Assertion failed: {:?} ≉ {:?} (±{})", $val1, $val2, epsilon)
        }
    }};
    ($val1:expr, $val2:expr) => {
        $crate::utils::approx::assert_approx_ne!(
            $val1,
            $val2,
            $crate::utils::approx::Epsilon::default()
        )
    };
}

#[allow(unused)]
pub(crate) use assert_approx_ne;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_zero_epsilon() {
        assert!(approx_eq!(1.0_f32, 1.0_f32, 0.0));

        assert!(approx_ne!(1.0_f32, 1.0_f32 + f32::EPSILON, 0.0));
        assert!(approx_ne!(1.0_f32, 1.0_f32 - f32::EPSILON, 0.0));
    }

    #[test]
    fn f32_default_epsilon() {
        assert!(approx_eq!(1.0_f32, 1.0_f32));

        assert!(approx_eq!(1.0_f32, 1.0_f32 + f32::EPSILON));
        assert!(approx_eq!(1.0_f32, 1.0_f32 - f32::EPSILON));

        assert!(approx_ne!(1.0_f32, 1.0_f32 + 2.0 * f32::EPSILON));
        assert!(approx_ne!(1.0_f32, 1.0_f32 - 2.0 * f32::EPSILON));
    }

    #[test]
    fn f32_double_epsilon() {
        assert!(approx_eq!(1.0_f32, 1.0_f32, 2.0));

        assert!(approx_eq!(
            1.0_f32,
            1.0_f32 + f32::EPSILON,
            2.0 * f32::EPSILON
        ));
        assert!(approx_eq!(
            1.0_f32,
            1.0_f32 - f32::EPSILON,
            2.0 * f32::EPSILON
        ));

        assert!(approx_eq!(
            1.0_f32,
            1.0_f32 + 2.0 * f32::EPSILON,
            2.0 * f32::EPSILON
        ));
        assert!(approx_eq!(
            1.0_f32,
            1.0_f32 - 2.0 * f32::EPSILON,
            2.0 * f32::EPSILON
        ));

        assert!(approx_ne!(
            1.0_f32,
            1.0_f32 + 3.0 * f32::EPSILON,
            2.0 * f32::EPSILON
        ));
        assert!(approx_ne!(
            1.0_f32,
            1.0_f32 - 3.0 * f32::EPSILON,
            2.0 * f32::EPSILON
        ));
    }

    #[test]
    fn f64_zero_epsilon() {
        assert!(approx_eq!(1.0_f64, 1.0_f64, 0.0));

        assert!(approx_ne!(1.0_f64, 1.0_f64 + f64::EPSILON, 0.0));
        assert!(approx_ne!(1.0_f64, 1.0_f64 - f64::EPSILON, 0.0));
    }

    #[test]
    fn f64_default_epsilon() {
        assert!(approx_eq!(1.0_f64, 1.0_f64));

        assert!(approx_eq!(1.0_f64, 1.0_f64 + f64::EPSILON));
        assert!(approx_eq!(1.0_f64, 1.0_f64 - f64::EPSILON));

        assert!(approx_ne!(1.0_f64, 1.0_f64 + 2.0 * f64::EPSILON));
        assert!(approx_ne!(1.0_f64, 1.0_f64 - 2.0 * f64::EPSILON));
    }

    #[test]
    fn f64_double_epsilon() {
        assert!(approx_eq!(1.0_f64, 1.0_f64, 2.0));

        assert!(approx_eq!(
            1.0_f64,
            1.0_f64 + f64::EPSILON,
            2.0 * f64::EPSILON
        ));
        assert!(approx_eq!(
            1.0_f64,
            1.0_f64 - f64::EPSILON,
            2.0 * f64::EPSILON
        ));

        assert!(approx_eq!(
            1.0_f64,
            1.0_f64 + 2.0 * f64::EPSILON,
            2.0 * f64::EPSILON
        ));
        assert!(approx_eq!(
            1.0_f64,
            1.0_f64 - 2.0 * f64::EPSILON,
            2.0 * f64::EPSILON
        ));

        assert!(approx_ne!(
            1.0_f64,
            1.0_f64 + 3.0 * f64::EPSILON,
            2.0 * f64::EPSILON
        ));
        assert!(approx_ne!(
            1.0_f64,
            1.0_f64 - 3.0 * f64::EPSILON,
            2.0 * f64::EPSILON
        ));
    }
}
