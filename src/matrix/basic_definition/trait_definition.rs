/// nessecary trait to calculate the norm of self
pub trait Normable :
    PartialOrd
{
    fn norm(&self) -> Self;
    fn square(&self) -> Self;
    fn square_root(&self) -> Self;
}

/// trait to define if self is close to zero
pub trait Zero {
    fn close_to_zero(&self) -> bool;
}

/// trait to define a complex conjugate
pub trait Complexe {
    fn conjugate(&self) -> Self;
}