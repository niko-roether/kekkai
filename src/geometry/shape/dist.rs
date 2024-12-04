use crate::geometry::Scalar;

use super::{Point, Segment};

pub fn point_to_point(a: Point, b: Point) -> Scalar {
    (b - a).norm()
}

pub fn point_to_segment(p: Point, segment: &Segment) -> Scalar {
    segment.distance_to_point(p)
}

pub fn segment_to_segment(s1: &Segment, s2: &Segment) -> Scalar {
    let d1 = s1.distance_to_point(s2.start);
    let d2 = s1.distance_to_point(s2.end);
    let d3 = s2.distance_to_point(s1.start);
    let d4 = s2.distance_to_point(s1.end);
    d1.min(d2).min(d3).min(d4)
}

#[cfg(test)]
mod tests {
    use crate::utils::approx::assert_approx_eq;

    use super::*;

    #[test]
    fn point_to_point() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 3.0);
        assert_approx_eq!(super::point_to_point(p1, p2), Scalar::sqrt(5.0));
    }

    #[test]
    fn point_to_segment() {
        let p = Point::new(3.0, 2.0);
        let segment = Segment::new(Point::new(2.0, 1.0), Point::new(4.0, 2.0));
        assert_approx_eq!(super::point_to_segment(p, &segment), 0.3826834323650898)
    }

    #[test]
    fn segment_to_segment_case_1() {
        let s1 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        let s2 = Segment::new(Point::new(2.0, 2.0), Point::new(4.0, 3.0));
        assert_approx_eq!(super::segment_to_segment(&s1, &s2), 0.5);
    }

    #[test]
    fn segment_to_segment_case_2() {
        let s1 = Segment::new(Point::new(2.0, 2.0), Point::new(4.0, 3.0));
        let s2 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        assert_approx_eq!(super::segment_to_segment(&s1, &s2), 0.5);
    }

    #[test]
    fn segment_to_segment_parallel() {
        let s1 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        let s2 = Segment::new(Point::new(2.0, 2.0), Point::new(3.0, 4.0));
        assert_approx_eq!(super::segment_to_segment(&s1, &s2), 0.5);
    }
}
