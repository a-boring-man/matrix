use std::ops::{Add, Sub, Mul, Div};

// ---------------------------- Scalar trait definition --------------------------

pub trait Scalar :
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Clone + Sized + Copy + PartialEq
{
}

pub trait Normable :
    std::iter::Sum + PartialOrd
{
    fn norm(&self) -> Self;
    fn square(&self) -> Self;
    fn square_root(&self) -> Self;
}

pub trait Complexe {
    fn conjugate(&self) -> Self;
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

impl<T> Scalar for T
where
    T: Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + Clone
    + Copy
    + PartialEq
{
}
