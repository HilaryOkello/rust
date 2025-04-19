use std::ops::{Add, Mul};
use std::fmt::Debug;

// Define the Scalar trait
// A scalar must support addition, multiplication, copying, and have a default zero value.
pub trait Scalar: Add<Output = Self> + Mul<Output = Self> + Copy + Default + Debug {}

// Blanket implementation for any type that satisfies the constraints
impl<T> Scalar for T where T: Add<Output = T> + Mul<Output = T> + Copy + Default + Debug {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut sum = T::default();
        for i in 0..self.0.len() {
            sum = sum + self.0[i] * other.0[i];
        }
        Some(sum)
    }
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result_vec: Vec<T> = self.0.iter()
            .zip(other.0.iter())
            .map(|(&s, &o)| s + o)
            .collect();

        Some(Vector(result_vec))
    }
}
