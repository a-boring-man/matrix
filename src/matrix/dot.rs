use super::definition::{Scalar, Vector};

impl<K: Scalar + std::iter::Sum<<K as std::ops::Mul>::Output>> Vector<K> {
	pub fn dot(&self, v: Vector<K>) ->K {
		self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() * b.clone()).sum()
	}
}