use core::f32;
use std::{iter, slice};

use nalgebra::Vector2;

use crate::utils::distance_to_line_segment;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Vector2<f32>>,
}

fn winding_number_contribution(pos: Vector2<f32>, start: Vector2<f32>, end: Vector2<f32>) -> i32 {
    let interpolation = (pos.y - start.y) / (end.y - start.y);
    if !(0.0..=1.0).contains(&interpolation) {
        return 0;
    }

    let intersection = start + interpolation * (end - start);
    if intersection.x < pos.x {
        return 0;
    }

    if start.y < end.y {
        1
    } else {
        -1
    }
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        let mut unsinged_dist = f32::INFINITY;
        let mut winding_number = 0_i32;
        for (start, end) in self.sides() {
            winding_number += winding_number_contribution(pos, start, end);
            unsinged_dist = f32::min(unsinged_dist, distance_to_line_segment(pos, start, end));
        }

        if winding_number == 0 {
            unsinged_dist
        } else {
            -unsinged_dist
        }
    }

    fn sides(&self) -> impl Iterator<Item = (Vector2<f32>, Vector2<f32>)> + '_ {
        LineIterator::new(&self.vertices)
    }
}

impl From<Vec<Vector2<f32>>> for Polygon {
    fn from(vertices: Vec<Vector2<f32>>) -> Self {
        Self { vertices }
    }
}

impl FromIterator<Vector2<f32>> for Polygon {
    fn from_iter<T: IntoIterator<Item = Vector2<f32>>>(iter: T) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

enum LineIterator<'a> {
    Degenerate,
    Normal {
        first_vertex: Vector2<f32>,
        vertex_iter: iter::Peekable<slice::Iter<'a, Vector2<f32>>>,
    },
}

impl<'a> LineIterator<'a> {
    fn new(vertices: &'a [Vector2<f32>]) -> Self {
        if vertices.len() < 2 {
            return Self::Degenerate;
        }
        Self::Normal {
            first_vertex: *vertices.first().unwrap(),
            vertex_iter: vertices.iter().peekable(),
        }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = (Vector2<f32>, Vector2<f32>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Degenerate => None,
            Self::Normal {
                first_vertex,
                vertex_iter,
            } => {
                let current_vertex = vertex_iter.next()?;
                if let Some(next_vertex) = vertex_iter.peek() {
                    Some((*current_vertex, **next_vertex))
                } else {
                    Some((*current_vertex, *first_vertex))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use super::*;

    #[test]
    fn should_return_correct_signed_distances_for_convex_polygons() {
        // when
        let polygon = Polygon::from(vec![
            vector![-2.0, -1.0],
            vector![2.0, -4.0],
            vector![5.0, -1.0],
            vector![3.0, 3.0],
            vector![-1.0, 2.0],
        ]);

        // then
        assert_eq!(polygon.signed_distance(vector![1.0, 4.0]), 1.4552137);
        assert_eq!(polygon.signed_distance(vector![3.0, -3.0]), 0.0);
        assert_eq!(polygon.signed_distance(vector![2.0, 1.0]), -1.6977493);
    }

    #[test]
    fn should_return_correct_signed_distances_for_simple_polygons() {
        // when
        let polygon = Polygon::from(vec![
            vector![-2.0, -1.0],
            vector![2.0, -3.0],
            vector![4.0, 1.0],
            vector![1.0, 0.0],
            vector![-1.0, 3.0],
        ]);

        // then
        assert_eq!(polygon.signed_distance(vector![1.0, 1.0]), 0.5547002);
        assert_eq!(polygon.signed_distance(vector![3.0, -1.0]), 0.0);
        assert_eq!(polygon.signed_distance(vector![-1.0, 1.0]), -0.4850712);
    }

    #[test]
    fn should_return_correct_signed_distances_for_complex_polygons() {
        // when
        let polygon = Polygon::from(vec![
            vector![-2.0, 0.0],
            vector![2.0, -2.0],
            vector![2.0, 0.0],
            vector![-2.0, -2.0],
        ]);

        // then
        assert_eq!(polygon.signed_distance(vector![0.0, 0.0]), 0.8944272);
        assert_eq!(polygon.signed_distance(vector![0.0, -1.0]), 0.0);
        assert_eq!(polygon.signed_distance(vector![-1.0, -1.0]), -0.4472136);
    }
}
