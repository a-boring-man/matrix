use std::{ops::{Add, Mul}, fmt::Display};

use crate::matrix::basic_definition::{trait_definition::Complexe, definition::Vector};

impl<K: Complexe + Default + Display + Copy + Add<Output = K> + Mul<Output = K>, const L: usize> Vector<K, L> {
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