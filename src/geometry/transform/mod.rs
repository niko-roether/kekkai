mod scaled_rotation;
mod similarity;
mod translation;

use std::ops::Mul;

pub use scaled_rotation::*;
pub use similarity::*;
pub use translation::*;

use crate::utils::approx::ApproxEq;

use super::Vector;

pub trait Transform: ApproxEq + Mul<Self> + Mul<Vector> + Sized {}
