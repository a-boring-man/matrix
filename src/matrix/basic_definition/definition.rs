// -------------------------- Basic implementation ------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct matrix<K, const R: usize, const C: usize> (pub [[K; C]; R]);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vector<K, const L: usize> (pub [K; L]);