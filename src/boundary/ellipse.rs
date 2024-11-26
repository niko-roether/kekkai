use nalgebra::Vector2;

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

    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        let avg_dist = ((pos - self.focus_1).magnitude() + (pos - self.focus_2).magnitude()) / 2.0;
        avg_dist - self.average_distance
    }
}
