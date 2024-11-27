use core::f32;
use std::{iter, slice};

use nalgebra::Vector2;

use crate::utils::{self, distance_to_line_segment};

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
    pub fn new(vertices: Vec<Vector2<f32>>) -> Self {
        Self { vertices }
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

impl FromIterator<Vector2<f32>> for Polygon {
    fn from_iter<T: IntoIterator<Item = Vector2<f32>>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
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
