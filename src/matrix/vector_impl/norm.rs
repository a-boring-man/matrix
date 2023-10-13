use crate::matrix::basic_definition::{trait_definition::{Scalar, Normable}, definition::{vector}};

use std::ops::Add;

impl<K: Normable + Default + Copy + Add<Output = K>, const L: usize> vector<K, L> {
	/// Norme 1 return 0 if vector is empty
	pub fn norm_taxicab(&self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			result = result + self.0[i].norm();
		}
		result
	}

	/// Norm 2 return 0 if the vector is empty
	pub fn norm_euclidean(&self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			result = result + self.0[i].norm().square();
		}
		result.square_root()
	}

	pub fn norm_supremum(&self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			match self.0[i].norm() > result {
				true => result = self.0[i].norm(),
				false => {},
			}
		}
		result
	}
}