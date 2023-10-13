// -------------------------- Basic implementation ------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix<K, const R: usize, const C: usize> (pub [[K; C]; R]);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector<K, const L: usize> (pub [K; L]);