
use std::ops::Mul;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix};

impl<K: Scalar> Matrix<K> where for<'a> &'a K: Mul<&'a K, Output = K>{
	pub fn determinant_2d(&self) -> K {
		if self.len() != 4 || !self.is_square() {
			panic!("wrong matrix dimension in determinant calculation");
		}
		&self.data[0] * &self.data[2] - &self.data[1] * &self.data[3]
	}

	pub fn determinant_3d(&self) -> K {
		if self.data.len() != 9 || !self.is_square() {
			panic!("wrong matrix dimension in determinant calculation");
		}
		let m = &self.data;
		&(&m[0] * &m[4]) * &m[8] + &(&m[1] * &m[5]) * &m[6] + &(&m[2] * &m[3]) * &m[7] - &(&m[2] * &m[4]) * &m[6] - &(&m[1] * &m[3]) * &m[8] - &(&m[0] * &m[5]) * &m[7]
	}
}