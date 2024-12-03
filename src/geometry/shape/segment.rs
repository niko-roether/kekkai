use crate::{
    geometry::{transform::Transform, Scalar, Vector},
    utils::approx::ApproxEq,
};

use super::{Point, Shape};

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
}

impl ApproxEq for Segment {
    type Epsilon = <Point as ApproxEq>::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.start.approx_eq(&other.start, epsilon) && self.end.approx_eq(&other.end, epsilon)
    }
}

impl Shape for Segment {
    fn transform(&mut self, t: impl Transform) {
        self.start.transform(t.clone());
        self.end.transform(t);
    }
}

#[cfg(test)]
mod tests {
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
    fn transform() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 4.0);
        let mut segment = Segment::new(p1, p2);

        segment.transform(Translation::new(1.0, 0.0));
        assert_approx_eq!(segment.start, Point::new(2.0, 2.0));
        assert_approx_eq!(segment.end, Point::new(3.0, 4.0));
    }
}
