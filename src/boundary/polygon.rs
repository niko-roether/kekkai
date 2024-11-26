use core::f32;
use std::{iter, slice};

use nalgebra::Vector2;

use crate::utils;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Vector2<f32>>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector2<f32>>) -> Self {
        Self { vertices }
    }

    pub fn signed_distance(&self, pos: Vector2<f32>) -> f32 {
        let mut dist = f32::NEG_INFINITY;
        for (line_1, line_2) in self.sides() {
            let line_dist = utils::signed_distance_to_line(pos, line_1, line_2);
            dist = f32::max(dist, line_dist);
        }
        dist
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
