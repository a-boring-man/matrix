use super::definition::{Scalar, Matrix};
use std::ops::AddAssign;

impl<K: Scalar + Default + AddAssign> Matrix<K> {
	pub fn trace(&self) -> K {
		let (nbr_col, _) = self.get_shape();
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