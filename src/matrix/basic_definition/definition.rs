use super::trait_definition::Scalar;

// -------------------------- Basic implementation ------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<K: Scalar> {
    pub(in crate) data: Vec::<K>,
    pub(in crate) col: u8,
    pub(in crate) row: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<K: Scalar> {
    pub(in crate) v: Vec::<K>,
}