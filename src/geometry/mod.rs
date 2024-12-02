mod vector;

#[cfg(feature = "f32")]
pub type Scalar = f32;

#[cfg(not(feature = "f32"))]
pub type Scalar = f64;

pub use vector::*;
