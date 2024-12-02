mod point;

use super::transform::Transform;

pub use point::*;

pub trait Shape {
    fn transform(&mut self, t: impl Transform);
}
