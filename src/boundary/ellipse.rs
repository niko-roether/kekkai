use nalgebra::{vector, Vector2};

#[derive(Debug, Clone)]
pub struct Ellipse {
    focus_1: Vector2<f32>,
    focus_2: Vector2<f32>,
    average_distance: f32,
}

impl Ellipse {
    pub fn new(focus_1: Vector2<f32>, focus_2: Vector2<f32>, average_distance: f32) -> Self {
        Self {
            focus_1,
            focus_2,
            average_distance,
        }
    }

    pub fn from_radii(
        center: Vector2<f32>,
        minor_radius: f32,
        major_radius: f32,
        rotation: f32,
    ) -> Self {
        let focus_distance = (major_radius.powi(2) - minor_radius.powi(2)).sqrt();
        let focus_direction = vector![rotation.cos(), rotation.sin()];
        let focus_1 = center + focus_distance * focus_direction;
        let focus_2 = center - focus_distance * focus_direction;
        Self::new(focus_1, focus_2, major_radius)
    }

    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        let avg_dist = ((pos - self.focus_1).magnitude() + (pos - self.focus_2).magnitude()) / 2.0;
        avg_dist - self.average_distance
    }
}
