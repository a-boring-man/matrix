use std::ops::{Add, Mul};

use crate::matrix::basic_definition::{trait_definition::{Scalar, Complexe, CanDoaDotProduct, CanDoaDotProductComplex}, definition::{Vector, vector}, complex::Complex};

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

impl<K: Copy + Default + Add<Output = K> + Mul<Output = K>, const L: usize> CanDoaDotProduct<K> for vector<K, L> {
    fn dot(&self, other: Self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			unsafe {
				result = result + *self.0.get_unchecked(i) * *other.0.get_unchecked(i);
			}
		}
		result
    }
}

impl<K: Copy + Default + Complexe + Add<Output = K> + Mul<Output = K>, const L: usize> CanDoaDotProductComplex<K> for vector<K, L> {
    fn dot(&self, other: Self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			unsafe {
				result = result + *self.0.get_unchecked(i) * other.0.get_unchecked(i).conjugate();
			}
		}
		result
    }
}