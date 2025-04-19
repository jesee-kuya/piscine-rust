use std::ops::{Add, Sub, Mul, Div};

// Trait Scalar with required arithmetic ops + zero and one
pub trait Scalar:
    Copy + Clone + PartialEq + Add<Output = Self> + Sub<Output = Self> +
    Mul<Output = Self> + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

// Implement Scalar for various types
impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u64 {
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

// Vector struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

// Implement Add for Vector
impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(result))
    }
}

// Implement Vector functions
impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut sum = T::zero();
        for i in 0..self.0.len() {
            sum = sum + self.0[i] * other.0[i];
        }

        Some(sum)
    }
}
