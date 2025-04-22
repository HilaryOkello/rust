// matrix/src/lib.rs
use lalgebra_scalar::Scalar; 

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
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