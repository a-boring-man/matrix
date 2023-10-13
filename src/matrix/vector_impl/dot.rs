use std::ops::{Add, Mul};

use crate::matrix::basic_definition::{trait_definition::Complexe, definition::Vector};

impl<K: Default + Copy + Add<Output = K> + Mul<Output = K>, const L: usize> Vector<K, L> {
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
}
impl<K: Complexe + Default + Copy + Add<Output = K> + Mul<Output = K>, const L: usize> Vector<K, L> {
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