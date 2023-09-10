use std::usize;

use super::trait_definition::Scalar;

// -------------------------- Basic implementation ------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<K: Scalar> {
    pub(in crate) data: Vec::<K>,
    pub(in crate) col: u8,
    pub(in crate) row: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct matrix<K, const R: usize, const C: usize> {
    pub e: [[K; C]; R],
}

#[derive(Clone, PartialEq, Debug)]
pub struct vector<K, const L: usize> {
    pub e: [K; L],
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<K: Scalar> {
    pub(in crate) v: Vec::<K>,
}