
use std::ops::Mul;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix};

impl<K: Scalar> Matrix<K> where for<'a> &'a K: Mul<&'a K, Output = K>{

	fn  get_sub_matrix(&self, i: u8, j: u8) -> Matrix<K> {
		let mut result = Vec::with_capacity((self.row -1).pow(2) as usize);
		for r in 0..self.row {
			for c in 0..self.col {
				if r != j && c != i {
					result.push(self.data[self.linear_index(c, r) as usize])
				}
			}
		}
		Matrix { data: result, col: self.col - 1, row: self.row - 1 }
	}

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

	pub fn determinant_4d(&self) -> K {
		if self.len() != 16 || !self.is_square() {
			panic!("wrong matrix dimension in determinant calculation");
		}
		self.data[0] * self.get_sub_matrix(0, 0).determinant_3d() - self.data[self.linear_index(0, 1) as usize] * self.get_sub_matrix(0, 1).determinant_3d() + self.data[self.linear_index(0, 2) as usize] * self.get_sub_matrix(0, 2).determinant_3d() - self.data[self.linear_index(0, 3) as usize] * self.get_sub_matrix(0, 3).determinant_3d()
	}
}