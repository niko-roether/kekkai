use std::borrow::Cow;

use nalgebra::{Affine2, Vector2};

use super::Boundary;

#[derive(Debug, Clone)]
pub struct Composite<'a> {
    components: Vec<CompositeComponent<'a>>,
}

impl<'a> Composite<'a> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, boundary: &'a Boundary<'a>, transform: Affine2<f32>) {
        self.components.push(CompositeComponent {
            boundary: Cow::Borrowed(boundary),
            inverse_transform: transform.inverse(),
        });
    }

    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        let mut distance = 0.0;
        for component in &self.components {
            let transformed_pos = component.inverse_transform * pos;
            let sd = component.boundary.signed_distance(transformed_pos);
            distance = f32::min(distance, sd);
        }
        distance
    }
}

impl<'a> Default for Composite<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct CompositeComponent<'a> {
    boundary: Cow<'a, Boundary<'a>>,
    inverse_transform: Affine2<f32>,
}
