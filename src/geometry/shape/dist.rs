use crate::geometry::Scalar;

use super::{Chain, Point, Segment};

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

pub fn point_to_chain(p: Point, chain: &Chain) -> Scalar {
    chain.distance_to_point(p)
}

pub fn segment_to_chain(s: &Segment, c: &Chain) -> Scalar {
    c.segments()
        .map(|cs| segment_to_segment(s, &cs))
        .fold(Scalar::INFINITY, Scalar::min)
}

pub fn chain_to_chain(c1: &Chain, c2: &Chain) -> Scalar {
    c1.segments()
        .map(|s| segment_to_chain(&s, c2))
        .fold(Scalar::INFINITY, Scalar::min)
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

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
        assert_approx_eq!(
            super::point_to_segment(p, &segment),
            1.0 / Scalar::sqrt(5.0)
        )
    }

    #[test]
    fn segment_to_segment_case_1() {
        let s1 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        let s2 = Segment::new(Point::new(2.0, 2.0), Point::new(4.0, 3.0));
        assert_approx_eq!(super::segment_to_segment(&s1, &s2), 1.0 / Scalar::sqrt(5.0));
    }

    #[test]
    fn segment_to_segment_case_2() {
        let s1 = Segment::new(Point::new(2.0, 2.0), Point::new(4.0, 3.0));
        let s2 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        assert_approx_eq!(super::segment_to_segment(&s1, &s2), 1.0 / Scalar::sqrt(5.0));
    }

    #[test]
    fn segment_to_segment_parallel() {
        let s1 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        let s2 = Segment::new(Point::new(2.0, 2.0), Point::new(3.0, 4.0));
        assert_approx_eq!(super::segment_to_segment(&s1, &s2), 1.0 / Scalar::sqrt(5.0));
    }

    #[test]
    fn point_to_chain() {
        let p = Point::new(3.0, 2.0);
        let c = Chain::new(vec![
            Point::new(3.0, 0.0),
            Point::new(2.0, 1.0),
            Point::new(4.0, 2.0),
            Point::new(1.0, -1.0),
        ]);
        assert_approx_eq!(super::point_to_chain(p, &c), 1.0 / Scalar::sqrt(5.0));
    }

    #[test]
    fn segment_to_chain() {
        let s = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        let c = Chain::new(vec![
            Point::new(1.0, 4.0),
            Point::new(2.0, 2.0),
            Point::new(4.0, 3.0),
            Point::new(5.0, 2.0),
        ]);
        assert_approx_eq!(super::segment_to_chain(&s, &c), 1.0 / Scalar::sqrt(5.0));
    }

    #[test]
    fn chain_to_chain() {
        let c1 = Chain::new(vec![
            Point::new(-1.0, -2.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 3.0),
            Point::new(-1.0, 3.0),
        ]);
        let c2 = Chain::new(vec![
            Point::new(1.0, 4.0),
            Point::new(2.0, 2.0),
            Point::new(4.0, 3.0),
            Point::new(5.0, 2.0),
        ]);
        assert_approx_eq!(super::chain_to_chain(&c1, &c2), 1.0 / Scalar::sqrt(5.0));
    }

    #[bench]
    fn point_to_point_bench(b: &mut Bencher) {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 3.0);

        b.iter(|| black_box(super::point_to_point(p1, p2)));
    }

    #[bench]
    fn point_to_segment_bench(b: &mut Bencher) {
        let p = Point::new(1.0, 2.0);
        let s = Segment::new(Point::new(2.0, 1.0), Point::new(4.0, 2.0));

        b.iter(|| black_box(super::point_to_segment(p, &s)));
    }

    #[bench]
    fn segment_to_segment_bench(b: &mut Bencher) {
        let s1 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 3.0));
        let s2 = Segment::new(Point::new(2.0, 2.0), Point::new(4.0, 3.0));

        b.iter(|| black_box(super::segment_to_segment(&s1, &s2)));
    }

    #[bench]
    fn point_to_single_point_chain_bench(b: &mut Bencher) {
        let p = Point::new(1.0, 2.0);
        let c = Chain::from(Point::new(2.0, 1.0));

        b.iter(|| black_box(super::point_to_chain(p, &c)));
    }

    #[bench]
    fn point_to_single_segment_chain_bench(b: &mut Bencher) {
        let p = Point::new(1.0, 2.0);
        let c = Chain::from(Segment::new(Point::new(2.0, 1.0), Point::new(4.0, 2.0)));

        b.iter(|| black_box(super::point_to_chain(p, &c)));
    }

    #[bench]
    fn point_to_10_segment_chain_bench(b: &mut Bencher) {
        let p = Point::new(1.0, 2.0);
        let c = Chain::new(vec![
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(5.0, 0.0),
            Point::new(6.0, 0.0),
            Point::new(7.0, 0.0),
            Point::new(8.0, 0.0),
            Point::new(9.0, 0.0),
            Point::new(10.0, 0.0),
        ]);

        b.iter(|| black_box(super::point_to_chain(p, &c)));
    }
}
