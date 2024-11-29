use nalgebra::{vector, Vector2};

#[derive(Debug, Clone)]
pub struct Circle {
    center: Vector2<f32>,
    radius: f32,
}

impl Circle {
    pub fn new(center: Vector2<f32>, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn centered(radius: f32) -> Self {
        Self::new(vector![0.0, 0.0], radius)
    }

    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        (pos - self.center).magnitude() - self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_signed_distances() {
        // when
        let circle = Circle::new(vector![3.0, 0.0], 2.0);

        // then
        assert_eq!(circle.signed_distance(vector![1.0, 2.0]), 0.8284271);
        assert_eq!(circle.signed_distance(vector![3.0, 2.0]), 0.0);
        assert_eq!(circle.signed_distance(vector![2.0, 1.0]), -0.58578646);
    }
}
