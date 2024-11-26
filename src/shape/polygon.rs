use std::f32;

use anyhow::anyhow;
use nalgebra::Vector2;

use super::{ConvexShape, Shape};

pub trait Polygon: Shape {
    fn density_at(&self, point: Vector2<f32>) -> f32;

    fn density(&self) -> f32 {
        self.density_at(Vector2::zeros())
    }
}

/// A shape consisting of three or more vertices, connected by straight edges.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexPolygon<L = Vec<Vector2<f32>>> {
    vertices: L,
}

impl<L> ComplexPolygon<L>
where
    L: AsRef<[Vector2<f32>]>,
{
    pub fn new(vertices: L) -> Result<Self, anyhow::Error> {
        let num_vertices = vertices.as_ref().len();
        if num_vertices < 3 {
            return Err(anyhow!(
                "Polygons need at least 3 vertices; received {num_vertices}"
            ));
        }
        Ok(Self { vertices })
    }
}

impl<L> Polygon for ComplexPolygon<L>
where
    L: Clone + AsRef<[Vector2<f32>]> + AsMut<[Vector2<f32>]>,
{
    fn density_at(&self, point: Vector2<f32>) -> f32 {
        let mut angle_sum: f32 = 0.0;
        let mut prev_vertex_normal = (self.vertices.as_ref().last().unwrap() - point).normalize();
        for vertex in self.vertices.as_ref() {
            let vertex_normal = (vertex - point).normalize();
            let angle = f32::acos(Vector2::dot(&vertex_normal, &prev_vertex_normal));
            prev_vertex_normal = vertex_normal;
            angle_sum += angle;
        }
        angle_sum / f32::consts::TAU
    }
}

impl<L> Shape for ComplexPolygon<L>
where
    L: Clone + AsRef<[Vector2<f32>]> + AsMut<[Vector2<f32>]>,
{
    type ConvexHull = ConvexPolygon<L>;

    fn bounding_radius(&self) -> f32 {
        self.vertices
            .as_ref()
            .iter()
            .copied()
            .map(|v| v.norm_squared())
            .reduce(f32::max)
            .unwrap()
            .sqrt()
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.density_at(point) != 0.0
    }

    fn convex_hull(&self) -> Self::ConvexHull {
        todo!()
    }

    fn scale(&mut self, factor: f32) {
        self.vertices.as_mut().iter_mut().for_each(|v| *v *= factor);
    }
}

pub type ComplexNGon<const N: usize> = ComplexPolygon<[Vector2<f32>; N]>;

#[derive(Debug, Clone, PartialEq)]
pub struct SimplePolygon<L = Vec<Vector2<f32>>>(ComplexPolygon<L>);

impl<L> Shape for SimplePolygon<L>
where
    L: Clone + AsRef<[Vector2<f32>]> + AsMut<[Vector2<f32>]>,
{
    type ConvexHull = ConvexPolygon<L>;

    fn bounding_radius(&self) -> f32 {
        self.0.bounding_radius()
    }

    fn convex_hull(&self) -> Self::ConvexHull {
        todo!()
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        todo!()
    }

    fn scale(&mut self, factor: f32) {
        self.0.scale(factor);
    }
}

pub type SimpleNGon<const N: usize> = SimplePolygon<[Vector2<f32>; N]>;

#[derive(Debug, Clone, PartialEq)]
pub struct ConvexPolygon<L = Vec<Vector2<f32>>>(SimplePolygon<L>);

impl<L> Shape for ConvexPolygon<L>
where
    L: Clone + AsRef<[Vector2<f32>]> + AsMut<[Vector2<f32>]>,
{
    type ConvexHull = Self;

    fn bounding_radius(&self) -> f32 {
        self.0.bounding_radius()
    }

    fn contains(&self, _point: Vector2<f32>) -> bool {
        todo!()
    }

    fn convex_hull(&self) -> Self::ConvexHull {
        self.clone()
    }

    fn scale(&mut self, factor: f32) {
        self.0.scale(factor)
    }
}

impl<L> ConvexShape for ConvexPolygon<L> where Self: Shape<ConvexHull = Self> {}

pub type ConvexNGon<const N: usize> = ConvexPolygon<[Vector2<f32>; N]>;

#[derive(Debug, Clone, PartialEq)]
pub struct RegularPolygon {
    outer_radius: f32,
    num_vertices: usize,
}

impl Shape for RegularPolygon {
    type ConvexHull = Self;

    fn bounding_radius(&self) -> f32 {
        self.outer_radius
    }

    fn contains(&self, _point: Vector2<f32>) -> bool {
        todo!()
    }

    fn convex_hull(&self) -> Self::ConvexHull {
        self.clone()
    }

    fn scale(&mut self, factor: f32) {
        self.outer_radius *= factor
    }
}

impl ConvexShape for RegularPolygon {}

#[derive(Debug, Clone, PartialEq)]
pub struct RegularNGon<const N: usize> {
    outer_radius: f32,
}

impl<const N: usize> Shape for RegularNGon<N> {
    type ConvexHull = Self;

    fn bounding_radius(&self) -> f32 {
        self.outer_radius
    }

    fn contains(&self, _point: Vector2<f32>) -> bool {
        todo!()
    }

    fn convex_hull(&self) -> Self::ConvexHull {
        self.clone()
    }

    fn scale(&mut self, factor: f32) {
        self.outer_radius *= factor
    }
}

impl<const N: usize> ConvexShape for RegularNGon<N> {}
