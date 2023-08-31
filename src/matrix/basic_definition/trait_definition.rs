use std::ops::{Add, Sub, Mul};

// ---------------------------- Scalar trait definition --------------------------

pub trait Scalar :
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Clone + Sized + Copy
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

impl<T> Scalar for T
where
    T: Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Clone
    + Copy
{
}
