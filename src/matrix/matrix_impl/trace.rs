use std::ops::{AddAssign, Add};

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Matrix, matrix}};

impl<K: Scalar + Default + AddAssign> Matrix<K> {
	pub fn trace(&self) -> K {
		let (nbr_col, _, _) = self.get_shape();
		if !self.is_square() || self.get_shape().0 == 0 {
			panic!("matrix is not squared, couldn't compute trace");
		}
		let mut result = K::default();
		for i in 0..nbr_col {
			let index = i * nbr_col + i;
			result += self.data[index as usize].clone();
		}
		result
	}
}

impl<K: Copy + Default + Add<Output = K>, const R: usize> matrix<K, R, R> {
	pub fn trace(&self) -> K {
		let mut result = K::default();
		result = self.0.iter().enumerate().fold(result, |acc, (i, v)| acc + v[i + 1]);
		result
	}
}