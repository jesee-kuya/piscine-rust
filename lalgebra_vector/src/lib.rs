use std::fmt::Debug;
use std::ops::{Add, Mul};

pub trait Scalar: Add<Output = Self> + Mul<Output = Self> + Clone + Debug + PartialEq + Eq + Sized {
    fn zero() -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(
            self.0.len(),
            other.0.len(),
            "Vectors must be of the same length for addition."
        );
        let summed: Vec<T> = self.0
            .into_iter()
            .zip(other.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Vector(summed)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut sum = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + (a.clone() * b.clone());
        }
        Some(sum)
    }
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
}