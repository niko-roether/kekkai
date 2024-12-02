#![allow(unused_macros, dead_code)]

use std::ops;

pub(crate) trait Tolerance: Copy {
    fn range_for(self, value: Self) -> ops::RangeInclusive<Self>;
}

impl Tolerance for f32 {
    fn range_for(self, value: Self) -> ops::RangeInclusive<Self> {
        let deviation = value.log2().floor().exp2() * f32::EPSILON * self;
        (value - deviation)..=(value + deviation)
    }
}

impl Tolerance for f64 {
    fn range_for(self, value: Self) -> ops::RangeInclusive<Self> {
        let deviation = value.log2().floor().exp2() * f64::EPSILON * self;
        (value - deviation)..=(value + deviation)
    }
}

pub(crate) trait ApproxEq {
    type Tolerance: Tolerance;

    fn approx_eq(&self, other: &Self, tolerance: Self::Tolerance) -> bool;

    fn approx_ne(&self, other: &Self, tolerance: Self::Tolerance) -> bool {
        !self.approx_eq(other, tolerance)
    }
}

impl ApproxEq for f32 {
    type Tolerance = f32;

    fn approx_eq(&self, other: &Self, tolerance: Self::Tolerance) -> bool {
        dbg!(tolerance.range_for(*self)).contains(other)
    }
}

impl ApproxEq for f64 {
    type Tolerance = f64;

    fn approx_eq(&self, other: &Self, tolerance: Self::Tolerance) -> bool {
        tolerance.range_for(*self).contains(other)
    }
}

impl<T: ApproxEq> ApproxEq for &T {
    type Tolerance = T::Tolerance;

    fn approx_eq(&self, other: &Self, tolerance: Self::Tolerance) -> bool {
        (**self).approx_eq(&**other, tolerance)
    }
}

macro_rules! approx_eq {
    ($val1:expr, $val2:expr, $tolerance:expr) => {
        $crate::utils::approx::ApproxEq::approx_eq(&$val1, &$val2, $tolerance)
    };
    ($val1:expr, $val2:expr) => {
        $crate::utils::approx::ApproxEq::approx_eq(&$val1, &$val2, 1.0)
    };
}

#[allow(unused)]
pub(crate) use approx_eq;

macro_rules! approx_ne {
    ($val1:expr, $val2:expr, $tolerance:expr) => {
        $crate::utils::approx::ApproxEq::approx_ne(&$val1, &$val2, $tolerance)
    };
    ($val1:expr, $val2:expr) => {
        $crate::utils::approx::ApproxEq::approx_ne(&$val1, &$val2, 1.0)
    };
}

#[allow(unused)]
pub(crate) use approx_ne;

macro_rules! assert_approx_eq {
    ($val1:expr, $val2:expr, $tolerance:expr) => {
        assert!($crate::utils::approx::approx_eq!($val1, $val2, $tolerance))
    };
    ($val1:expr, $val2:expr) => {
        assert!($crate::utils::approx::approx_eq!($val1, $val2))
    };
}

#[allow(unused)]
pub(crate) use assert_approx_eq;

macro_rules! assert_approx_ne {
    ($val1:expr, $val2:expr, $tolerance:expr) => {
        assert!($crate::utils::approx::approx_ne!($val1, $val2, $tolerance))
    };
    ($val1:expr, $val2:expr) => {
        assert!($crate::utils::approx::approx_ne!($val1, $val2))
    };
}

#[allow(unused)]
pub(crate) use assert_approx_ne;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_zero_tolerance() {
        assert!(approx_eq!(1.0_f32, 1.0_f32, 0.0));

        assert!(approx_ne!(1.0_f32, 1.0_f32 + f32::EPSILON, 0.0));
        assert!(approx_ne!(1.0_f32, 1.0_f32 - f32::EPSILON, 0.0));
    }

    #[test]
    fn f32_default_tolerance() {
        assert!(approx_eq!(1.0_f32, 1.0_f32));

        assert!(approx_eq!(1.0_f32, 1.0_f32 + f32::EPSILON));
        assert!(approx_eq!(1.0_f32, 1.0_f32 - f32::EPSILON));

        assert!(approx_ne!(1.0_f32, 1.0_f32 + 2.0 * f32::EPSILON));
        assert!(approx_ne!(1.0_f32, 1.0_f32 - 2.0 * f32::EPSILON));
    }

    #[test]
    fn f32_double_tolerance() {
        assert!(approx_eq!(1.0_f32, 1.0_f32, 2.0));

        assert!(approx_eq!(1.0_f32, 1.0_f32 + f32::EPSILON, 2.0));
        assert!(approx_eq!(1.0_f32, 1.0_f32 - f32::EPSILON, 2.0));

        assert!(approx_eq!(1.0_f32, 1.0_f32 + 2.0 * f32::EPSILON, 2.0));
        assert!(approx_eq!(1.0_f32, 1.0_f32 - 2.0 * f32::EPSILON, 2.0));

        assert!(approx_ne!(1.0_f32, 1.0_f32 + 3.0 * f32::EPSILON, 2.0));
        assert!(approx_ne!(1.0_f32, 1.0_f32 - 3.0 * f32::EPSILON, 2.0));
    }

    #[test]
    fn f64_zero_tolerance() {
        assert!(approx_eq!(1.0_f64, 1.0_f64, 0.0));

        assert!(approx_ne!(1.0_f64, 1.0_f64 + f64::EPSILON, 0.0));
        assert!(approx_ne!(1.0_f64, 1.0_f64 - f64::EPSILON, 0.0));
    }

    #[test]
    fn f64_default_tolerance() {
        assert!(approx_eq!(1.0_f64, 1.0_f64));

        assert!(approx_eq!(1.0_f64, 1.0_f64 + f64::EPSILON));
        assert!(approx_eq!(1.0_f64, 1.0_f64 - f64::EPSILON));

        assert!(approx_ne!(1.0_f64, 1.0_f64 + 2.0 * f64::EPSILON));
        assert!(approx_ne!(1.0_f64, 1.0_f64 - 2.0 * f64::EPSILON));
    }

    #[test]
    fn f64_double_tolerance() {
        assert!(approx_eq!(1.0_f64, 1.0_f64, 2.0));

        assert!(approx_eq!(1.0_f64, 1.0_f64 + f64::EPSILON, 2.0));
        assert!(approx_eq!(1.0_f64, 1.0_f64 - f64::EPSILON, 2.0));

        assert!(approx_eq!(1.0_f64, 1.0_f64 + 2.0 * f64::EPSILON, 2.0));
        assert!(approx_eq!(1.0_f64, 1.0_f64 - 2.0 * f64::EPSILON, 2.0));

        assert!(approx_ne!(1.0_f64, 1.0_f64 + 3.0 * f64::EPSILON, 2.0));
        assert!(approx_ne!(1.0_f64, 1.0_f64 - 3.0 * f64::EPSILON, 2.0));
    }
}
