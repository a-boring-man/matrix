use std::usize;

use super::trait_definition::Scalar;

// -------------------------- Basic implementation ------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<K: Scalar> {
    pub(in crate) data: Vec::<K>,
    pub(in crate) col: u8,
    pub(in crate) row: u8,
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug)]
pub struct matrix<K, const R: usize, const C: usize> (pub [[K; C]; R]);

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug)]
pub struct vector<K, const L: usize> (pub [K; L]);

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<K: Scalar> {
    pub(in crate) v: Vec::<K>,
}