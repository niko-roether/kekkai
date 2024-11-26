use nalgebra::Vector2;

pub fn signed_distance_to_line(
    point: Vector2<f32>,
    line_1: Vector2<f32>,
    line_2: Vector2<f32>,
) -> f32 {
    let line_diff = line_2 - line_1;
    (line_diff.perp(&point) + line_2.perp(&line_1)) / line_diff.magnitude()
}
