#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

use std::ops::{Add, Sub};

// Implement Add for ThreeDVector<T> where T supports Add and Copy
impl<T> Add for ThreeDVector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ThreeDVector {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

// Implement Sub for ThreeDVector<T> where T supports Sub and Copy
impl<T> Sub for ThreeDVector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ThreeDVector {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}
