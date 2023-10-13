use std::ops::{Add, Sub, Mul, Div};

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
    PartialOrd
{
    fn norm(&self) -> Self;
    fn square(&self) -> Self;
    fn square_root(&self) -> Self;
}

pub trait Zero {
    fn close_to_zero(&self) -> bool;
}
pub trait Complexe {
    fn conjugate(&self) -> Self;
}