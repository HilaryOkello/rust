// matrix/src/lib.rs

use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::default::Default;
use std::marker::Copy;
use std::cmp::PartialEq;


pub trait Scalar: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Copy + Default + PartialEq + Debug {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}


#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut data = Vec::with_capacity(row);
        for _r in 0..row {
            let mut row_vec = Vec::with_capacity(col);
            for _c in 0..col {
                row_vec.push(T::zero());
            }
            data.push(row_vec);
        }
        Matrix(data)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        if n == 0 {
            return Matrix(vec![]);
        }
        let mut data = Vec::with_capacity(n);
        for r in 0..n {
            let mut row_vec = Vec::with_capacity(n);
            for c in 0..n {
                if r == c {
                    row_vec.push(T::one());
                } else {
                    row_vec.push(T::zero());
                }
            }
            data.push(row_vec);
        }
        Matrix(data)
    }
}

// Implement the Add trait for Matrix
impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Option<Matrix<T>> {
        let self_rows = self.0.len();
        let rhs_rows = rhs.0.len();


        if self_rows != rhs_rows || (self_rows > 0 && self.0[0].len() != rhs.0[0].len()) {
            return None; // Dimensions do not match
        }

        // If self_rows is 0, then rhs_rows must also be 0, resulting in an empty matrix.
        if self_rows == 0 {
             return Some(Matrix(vec![]));
        }

        let cols = self.0[0].len();
        let mut result_data = Vec::with_capacity(self_rows);

        for r in 0..self_rows {
            let mut row_vec = Vec::with_capacity(cols);
            for c in 0..cols {
                row_vec.push(self.0[r][c] + rhs.0[r][c]);
            }
            result_data.push(row_vec);
        }

        Some(Matrix(result_data))
    }
}

// Implement the Sub trait for Matrix
impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Option<Matrix<T>> {
        let self_rows = self.0.len();
        let rhs_rows = rhs.0.len();

        // If either matrix has no rows, they are only compatible if both have no rows.
        // If rows > 0, we also need to check column count.
        if self_rows != rhs_rows || (self_rows > 0 && self.0[0].len() != rhs.0[0].len()) {
            return None; // Dimensions do not match
        }

        // If self_rows is 0, then rhs_rows must also be 0, resulting in an empty matrix.
         if self_rows == 0 {
             return Some(Matrix(vec![]));
        }

        let cols = self.0[0].len();
        let mut result_data = Vec::with_capacity(self_rows);

        for r in 0..self_rows {
            let mut row_vec = Vec::with_capacity(cols);
            for c in 0..cols {
                row_vec.push(self.0[r][c] - rhs.0[r][c]);
            }
            result_data.push(row_vec);
        }

        Some(Matrix(result_data))
    }
}