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

impl<K: Copy + Add<Output = K> + Mul<Output = K>, const L: usize> CanDoaDotProduct<K> for vector<K, L> {
    fn dot(&self, other: Self) -> K {
        if L == 0 {
			panic!("vector must not be of 0 dimension");
		}
		let mut result: K;
		unsafe {
			result = *self.0.get_unchecked(0) * *other.0.get_unchecked(0);
		}
		for i in 1..L {
			unsafe {
				result = result + *self.0.get_unchecked(i) * *other.0.get_unchecked(i);
			}
		}
		result
    }
}

impl<K: Copy + Complexe + Add<Output = K> + Mul<Output = K>, const L: usize> CanDoaDotProductComplex<K> for vector<K, L> {
    fn dot(&self, other: Self) -> K {
        if L == 0 {
			panic!("vector must not be of 0 dimension");
		}
		let mut result: K;
		unsafe {
			result = *self.0.get_unchecked(0) * other.0.get_unchecked(0).conjugate();
		}
		for i in 1..L {
			unsafe {
				result = result + *self.0.get_unchecked(i) * other.0.get_unchecked(i).conjugate();
			}
		}
		result
    }
}