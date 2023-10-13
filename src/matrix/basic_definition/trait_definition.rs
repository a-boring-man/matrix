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