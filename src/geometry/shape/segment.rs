use std::ops::{Mul, MulAssign};

use crate::{
    geometry::{transform::Similarity, Scalar, Vector},
    utils::approx::ApproxEq,
};

use super::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl Segment {
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub const fn as_vector(&self) -> Vector {
        self.start.vector_to(self.end)
    }

    pub fn length(&self) -> Scalar {
        self.as_vector().norm()
    }

    pub fn distance_to_point(&self, point: Point) -> Scalar {
        let segment_vec = self.as_vector();
        let t = (point - self.start).dot(segment_vec) / segment_vec.norm_sq();
        let t_clamped = t.clamp(0.0, 1.0);
        let closest_point = self.start + t_clamped * segment_vec;
        point.distance(closest_point)
    }

    pub fn horizontal_ray_intersection_type(&self, ray_start: Point) -> i32 {
        let intersection_t = (ray_start.y() - self.start.y()) / (self.end.y() - self.start.y());
        if !(0.0..=1.0).contains(&intersection_t) {
            return 0;
        }

        let intersection_x = self.start.x() + intersection_t * self.as_vector().x;
        if intersection_x < ray_start.x() {
            return 0;
        }

        if self.as_vector().y < 0.0 {
            -1
        } else {
            1
        }
    }

    pub fn transform(&mut self, t: &Similarity) {
        self.start *= t;
        self.end *= t;
    }
}

impl ApproxEq for Segment {
    type Epsilon = <Point as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.start.approx_eq(&other.start, epsilon) && self.end.approx_eq(&other.end, epsilon)
    }
}

impl Mul<Segment> for &Similarity {
    type Output = Segment;

    fn mul(self, mut rhs: Segment) -> Self::Output {
        rhs.transform(self);
        rhs
    }
}

impl MulAssign<&Similarity> for Segment {
    fn mul_assign(&mut self, rhs: &Similarity) {
        self.transform(rhs);
    }
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use crate::{
        geometry::{scalar, transform::Translation, vector},
        utils::approx::assert_approx_eq,
    };

    use super::*;

    #[test]
    fn as_vector() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 4.0);
        let segment = Segment::new(p1, p2);

        assert_approx_eq!(segment.as_vector(), vector!(1.0, 2.0));
    }

    #[test]
    fn length() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 4.0);
        let segment = Segment::new(p1, p2);

        assert_approx_eq!(segment.length(), Scalar::sqrt(5.0));
    }

    #[test]
    fn distance_to_point_along_line() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(1.0, 3.0);
        assert_approx_eq!(segment.distance_to_point(q), scalar::consts::FRAC_1_SQRT_2);
    }

    #[test]
    fn distance_to_end_point() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(-1.0, 2.0);
        assert_approx_eq!(segment.distance_to_point(q), 2.0);
    }

    #[test]
    fn horizontal_ray_not_intersectingy() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(1.5, 0.0);
        assert_eq!(segment.horizontal_ray_intersection_type(q), 0);
    }

    #[test]
    fn horizontal_ray_not_intersecting_x() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(3.0, 2.5);
        assert_eq!(segment.horizontal_ray_intersection_type(q), 0);
    }

    #[test]
    fn horizontal_ray_intersecting_upwards() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(1.5, 2.5);
        assert_eq!(segment.horizontal_ray_intersection_type(q), 1);
    }

    #[test]
    fn horizontal_ray_intersecting_downwards() {
        let p1 = Point::new(2.0, 3.0);
        let p2 = Point::new(1.0, 2.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(1.5, 2.5);
        assert_eq!(segment.horizontal_ray_intersection_type(q), -1);
    }

    #[test]
    fn transform() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 4.0);
        let mut segment = Segment::new(p1, p2);

        segment *= &Translation::new(1.0, 0.0).into();
        assert_approx_eq!(segment.start, Point::new(2.0, 2.0));
        assert_approx_eq!(segment.end, Point::new(3.0, 4.0));
    }

    #[bench]
    fn bench_distance_to_point(b: &mut Bencher) {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(1.0, 3.0);

        b.iter(|| black_box(segment.distance_to_point(q)));
    }

    #[bench]
    fn bench_horizontal_ray_intersection(b: &mut Bencher) {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);
        let segment = Segment::new(p1, p2);

        let q = Point::new(1.5, 2.5);

        b.iter(|| black_box(segment.horizontal_ray_intersection_type(q)));
    }
}
