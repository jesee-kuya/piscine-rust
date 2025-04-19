use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: 
    Copy + Clone + PartialEq + Add<Output = Self> + Sub<Output = Self> + 
    Mul<Output = Self> + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

// Implement Scalar for i64
impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

// Implement Scalar for other types as needed
impl Scalar for i32 {
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

/// A vector that can be added and multiplied by scalars
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Vec::new();
        let len = self.0.len();
        for i in 0..len {
            result.push(self.0[i] + other.0[i]);
        }
        Vector(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            return None; // Vectors of different lengths cannot be dot-multiplied
        }

        let mut sum = T::zero();
        for i in 0..self.0.len() {
            sum = sum + self.0[i] * other.0[i];
        }
        Some(Vector(vec![sum])) // Return as Vector<T> wrapped in Option
    }
}
