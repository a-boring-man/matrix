use crate::matrix::basic_definition::{trait_definition::{Scalar, Complexe}, definition::Vector};

impl<K: Scalar + std::iter::Sum<<K as std::ops::Mul>::Output>> Vector<K> {
	pub fn dot(&self, v: &Vector<K>) ->K {
		self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() * b.clone()).sum()
	}
}

impl<K: Scalar + Complexe + std::iter::Sum<<K as std::ops::Mul>::Output>> Vector<K> {
	pub fn dot_complex(&self, v: &Vector<K>) -> K {
		self.v.iter().zip(v.v.iter()).map(|(a, b)| *a * b.conjugate()).sum()
	}
}