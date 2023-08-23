use super::definition::{Scalar, Matrix};
use std::ops::Mul;

impl<K: Scalar> Matrix<K> where for<'a> &'a K: Mul<&'a K, Output = K>{
	pub fn determinant_2d(&self) -> K {
		if self.data.len() != 4 || !self.is_square() {
			panic!("wrong matrix dimension in determinant calculation");
		}
		&self.data[0] * &self.data[2] - &self.data[1] * &self.data[3]
	}
}