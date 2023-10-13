use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Matrix, matrix, vector}, error::MatrixError};
use num_traits::identities::One;
use std::{ops::{Mul, Div, Sub, Neg, Add}, fmt::Display};

impl<K: Scalar> Matrix<K> {

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

	fn determinant_2d(&self) -> K {
		self.data[0] * self.data[3] - self.data[1] * self.data[2]
	}

	fn determinant_3d(&self) -> K {
		let m = &self.data;
		m[0] * m[4] * m[8] + m[1] * m[5] * m[6] + m[2] * m[3] * m[7] - m[2] * m[4] * m[6] - m[1] * m[3] * m[8] - m[0] * m[5] * m[7]
	}

	fn determinant_4d(&self) -> K {
		self.data[0] * self.get_sub_matrix(0, 0).determinant_3d() - self.data[self.linear_index(0, 1) as usize] * self.get_sub_matrix(0, 1).determinant_3d() + self.data[self.linear_index(0, 2) as usize] * self.get_sub_matrix(0, 2).determinant_3d() - self.data[self.linear_index(0, 3) as usize] * self.get_sub_matrix(0, 3).determinant_3d()
	}

	pub fn determinant(&self) -> Result<K, MatrixError> {
		match self.get_shape() {
			(2, 2, 4) => Ok(Matrix::determinant_2d(self)),
			(3, 3, 9) => Ok(Matrix::determinant_3d(self)),
			(4, 4, 16) => Ok(Matrix::determinant_4d(self)),
			_ => Err(MatrixError::InvalidFormat)
		}
	}
}

impl<K: Default + Copy + Display + One + PartialEq + Mul<Output = K> + Add<Output = K> + Div<Output = K> + Sub<Output = K> + Neg<Output = K>, const R: usize> matrix<K, R, R> {

	pub fn determinant(&self) -> K {
		let find_first_row = |m: &matrix<K, R, R>, row: usize, col: usize| -> Option<usize> {
			for (i, vec) in m.0.iter().enumerate().skip(row) {
				if vec[col] != K::default() {
					return Some(i);
				}
			}
			None
		};

		let row_swap = |m: &mut matrix<K, R, R>, r1: usize, r2: usize| {
			for c in 0..R {
				let tmp: K = m.0[r1][c];
				m.0[r1][c] = m.0[r2][c];
				m.0[r2][c] = tmp;
			}
		};

		let mut copy = *self;
		let mut r = 0;
		let mut swap = false;
		for c in 0..R {
			if let Some(row1) = find_first_row(&copy, r, c) {
				if row1 != r {
					row_swap(&mut copy, row1, r);
					swap = !swap;
				}
				for r2 in r..R {
					if r2 != r {
						copy.0[r2] = (vector::from(copy.0[r2]) - vector::from(copy.0[r]) * (copy.0[r2][c] / copy.0[r][c])).0;
					}
					else {
						continue;
					}
				}	
				r += 1;
			}
			else {
				continue;
			}
		}
		println!("copy {}", copy);
		let mut det = copy.tracex();
		println!("tracex {}", det);
		if swap {
			det = -det;
		}
		det
	}
}