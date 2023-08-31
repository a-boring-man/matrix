use std::ops::AddAssign;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix};

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