mod point;
mod segment;

use super::transform::Transform;

pub use point::*;
pub use segment::*;

pub trait Shape {
    fn transform(&mut self, t: impl Transform);
}
