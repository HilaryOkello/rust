// Instructions
// Define the structure ThreeDVector, that represents a 3 dimensional vector.

// In physics, these vectors are represented as ai, bj and ck. a, b and c are real numbers. i, j and k represent the direction in the Cartesian plane, along the axises x, y and z respectively. Therefore, we use the fields i, j and k in the structure.

// Take a look how the operations Addition and Subtraction work for a 3 dimensional vector, and implement them by creating the std::ops::Add and std::ops::Sub traits.

use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
	pub i: T,
	pub j: T,
	pub k: T,
}

impl<T> Add for ThreeDVector<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl<T> Sub for ThreeDVector<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }

}