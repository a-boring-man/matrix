use std::ops::{Add, Sub, Mul, Div};

use super::definition::vector;

// ---------------------------- Scalar trait definition --------------------------

pub trait Scalar :
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sized + Copy + PartialEq
{
}

impl<T> Scalar for T
where
    T: Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + Sized
    + Copy
    + PartialEq
{
}

pub trait Normable :
    std::iter::Sum + PartialOrd
{
    fn norm(&self) -> Self;
    fn square(&self) -> Self;
    fn square_root(&self) -> Self;
}

impl Normable for f64 {
    fn norm(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn square_root(&self) -> Self {
        self.sqrt()
    }
}

impl Normable for f32 {
    fn norm(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn square_root(&self) -> Self {
        self.sqrt()
    }
}

pub trait Complexe {
    fn conjugate(&self) -> Self;
}

pub trait CanDoaDotProduct<K: Copy + Default + Mul<Output = K> + Add<Output = K>, const L: usize> {
    fn dot(first: &vector<K, L>, other: vector<K, L>) -> K {
        let mut result: K = K::default();
		for i in 0..L {
			unsafe {
				result = result + *first.0.get_unchecked(i) * *other.0.get_unchecked(i);
			}
		}
		result
    }
}