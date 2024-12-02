mod scaled_rotation;
mod similarity;
mod translation;

use std::ops::Mul;

pub use scaled_rotation::*;
pub use similarity::*;
pub use translation::*;

use super::Vector;

pub trait Transform: Clone + Mul<Vector, Output = Vector> + Sized {}
