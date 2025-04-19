use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::marker::{Sized, Copy};
use std::cmp::PartialEq;

/// A trait representing a scalar type, providing basic arithmetic operations
/// and identity elements for addition and multiplication.
pub trait Scalar: Add<Self, Output = Self>
                 + Sub<Self, Output = Self>
                 + Mul<Self, Output = Self>
                 + Div<Self, Output = Self>
                 + Sized
                 + Copy
                 + PartialEq
                 + Debug
{
    /// The concrete type of the scalar. For types implementing this trait,
    /// this will typically be `Self`.
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

// Implementations of the Scalar trait for various primitive numeric types.

impl Scalar for u32 {
    type Item = u32;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f32 {
    type Item = f32;

    #[inline]
    fn zero() -> Self::Item {
        0.0
    }

    #[inline]
    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = f64;

    #[inline]
    fn zero() -> Self::Item {
        0.0
    }

    #[inline]
    fn one() -> Self::Item {
        1.0
    }
}