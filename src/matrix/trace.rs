use super::definition::{Scalar, Matrix};

impl<K: Scalar> Matrix<K> {
	pub fn trace(&self) -> K {
		if !self.is_square() || self.get_shape().0 == 0 {
			panic!("matrix is not squared, couldn't compute trace");
		}
	}
}