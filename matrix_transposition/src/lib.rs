#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub(i32, i32), pub(i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    // Extract the elements from the matrix
    let Matrix((a, b), (c, d)) = m;

    Matrix((a, c), (b, d))
}