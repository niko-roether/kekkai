use nalgebra::{vector, Vector2};

#[derive(Debug, Clone)]
pub struct Ellipse {
    focus_1: Vector2<f32>,
    focus_2: Vector2<f32>,
    average_distance: f32,
}

impl Ellipse {
    pub fn new(focus_1: Vector2<f32>, focus_2: Vector2<f32>, major_axis: f32) -> Self {
        Self {
            focus_1,
            focus_2,
            average_distance: major_axis,
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

#[cfg(test)]
mod tests {
    use core::f32;

    use super::*;

    #[test]
    fn should_construct_from_radii() {
        // when
        let ellipse = Ellipse::from_radii(vector![0.0, 1.0], 2.0, 3.0, f32::consts::TAU / 8.0);

        // then
        assert_eq!(ellipse.focus_1, vector![1.5811388, 2.5811388]);
        assert_eq!(ellipse.focus_2, vector![-1.5811388, -0.58113885]);
        assert_eq!(ellipse.average_distance, 3.0);
    }

    #[test]
    fn should_return_correct_signed_distances() {
        // when
        let ellipse = Ellipse::new(vector![-1.0, 0.0], vector![1.0, 0.0], 2.0);

        // then
        assert_eq!(ellipse.signed_distance(vector![2.0, 0.0]), 0.0);
        assert_eq!(ellipse.signed_distance(vector![3.0, 0.0]), 1.0);
        assert_eq!(ellipse.signed_distance(vector![1.0, 0.0]), -1.0);
        assert_eq!(ellipse.signed_distance(vector![2.0, 2.0]), 0.92080975);
    }
}
