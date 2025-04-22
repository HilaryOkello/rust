use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::default::Default;
use std::marker::Copy;
use std::cmp::PartialEq;
use std::clone::Clone;


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


#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        // Represents a 1x1 matrix with the zero value
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

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() || self.0.first().map_or(false, |row| row.is_empty()) {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        if n >= self.number_of_rows() {
            panic!("Row index out of bounds: requested {}, but matrix has {} rows", n, self.number_of_rows());
        }
        // Return a copy of the row
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let num_rows = self.number_of_rows();
        if num_rows == 0 {
            return vec![];
        }
        let num_cols = self.number_of_cols();
        if n >= num_cols {
            panic!("Column index out of bounds: requested {}, but matrix has {} columns", n, num_cols);
        }

        let mut col_vec = Vec::with_capacity(num_rows);
        for r in 0..num_rows {
            col_vec.push(self.0[r][n]);
        }
        col_vec
    }
}

impl<T: Scalar> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Option<Matrix<T>> {
        let rows_a = self.number_of_rows();
        let cols_a = self.number_of_cols();
        let rows_b = rhs.number_of_rows();
        let cols_b = rhs.number_of_cols();

        // If dimensions are incompatible, return None
        if cols_a != rows_b {
            return None;
        }

        // If compatible, but resulting dimensions are zero, return Some(zero_matrix)
        if rows_a == 0 || cols_b == 0 {
             return Some(Matrix::zero(rows_a, cols_b));
         }

        let mut result = Matrix::zero(rows_a, cols_b);

        for i in 0..rows_a {
            for j in 0..cols_b {
                for k in 0..cols_a { // k iterates over the dimension being contracted (cols_a == rows_b)
                    result.0[i][j] = result.0[i][j] + self.0[i][k] * rhs.0[k][j];
                }
            }
        }

        Some(result)
    }
}