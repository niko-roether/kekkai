use nalgebra::Vector2;

pub fn distance_to_line_segment(
    point: Vector2<f32>,
    start: Vector2<f32>,
    end: Vector2<f32>,
) -> f32 {
    let segment_vec = end - start;
    let dir_vec = segment_vec / segment_vec.magnitude_squared();
    let interpolation = (point - start).dot(&dir_vec);
    let clamped_interpolation = interpolation.clamp(0.0, 1.0);
    let closest_point = start + clamped_interpolation * segment_vec;
    (point - closest_point).magnitude()
}
