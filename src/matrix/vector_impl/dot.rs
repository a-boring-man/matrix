use std::{ops::{Add, Mul}, fmt::Display};

use crate::matrix::basic_definition::{trait_definition::{Scalar, Complexe}, definition::{Vector, vector}};

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

impl<K: Complexe + Default + Display + Copy + Add<Output = K> + Mul<Output = K>, const L: usize> vector<K, L> {
	pub fn dot (&self, other: Self) -> K {
		let mut result =  K::default();
		for i in 0..L {
			unsafe {
				result = result + *self.0.get_unchecked(i)
				* *other.0.get_unchecked(i);
			}
		}
		result
	}

	pub fn complex_dot (&self, other: Self) -> K {
		let mut result =  K::default();
		for i in 0..L {
			unsafe {
				result = result + *self.0.get_unchecked(i)
				* other.0.get_unchecked(i).conjugate();
			}
		}
		result
	}
}