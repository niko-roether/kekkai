pub mod transform;
mod vector;

#[cfg(feature = "f32")]
pub mod scalar {
    pub use std::f32::*;
    pub type Scalar = f32;
}

#[cfg(not(feature = "f32"))]
pub mod scalar {
    pub use std::f64::*;
    pub type Scalar = f64;
}

pub use scalar::Scalar;

pub use vector::*;
