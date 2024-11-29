use nalgebra::{ArrayStorage, Dim, Scalar, Storage};

pub(crate) trait ApproxEq {
    type Epsilon: Clone;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool;

    fn approx_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !self.approx_eq(other, epsilon)
    }
}

impl ApproxEq for f32 {
    type Epsilon = f32;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (self - other).abs() <= epsilon
    }
}

impl<T> ApproxEq for [T]
where
    T: ApproxEq,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let mut iter_1 = self.as_ref().iter();
        let mut iter_2 = other.as_ref().iter();
        loop {
            match (iter_1.next(), iter_2.next()) {
                (None, None) => break,
                (None, Some(..)) | (Some(..), None) => return false,
                (Some(item_1), Some(item_2)) => {
                    if item_1.approx_ne(item_2, epsilon.clone()) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl<T> ApproxEq for nalgebra::Vector2<T>
where
    T: Scalar + ApproxEq,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.approx_eq(&other.x, epsilon.clone()) && self.y.approx_eq(&other.y, epsilon)
    }
}

macro_rules! approx_eq {
    ($received:expr, $expected:expr, $epsilon:expr) => {
        $crate::approx::ApproxEq::approx_eq($received, $expected, $epsilon)
    };
}

macro_rules! approx_ne {
    ($received:expr, $expected:expr, $epsilon:expr) => {
        $crate::approx::ApproxEq::approx_ne($received, $expected, $epsilon)
    };
}
