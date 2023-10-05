use core::fmt;
use std::ops::{Sub, Mul, Div, Add, Neg};
use num_traits::One;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Matrix, matrix, vector}, error::MatrixError};

impl<K: Scalar + Default + std::convert::From<i32> + One> Matrix<K> {
	pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
		let det = self.determinant()?;
		if det == K::default() {
			return Err(MatrixError::NotInversible);
		}
		let result = self.augment_matrix().row_echelon();
		Ok(result.extract_augment())
	}

	fn augment_matrix(&self) -> Matrix<K> {
		let (col, row, _) = self.get_shape();
		let id = Matrix::<K>::identity(col);
		let mut new_vec = Vec::with_capacity(2 * col as usize * row as usize);
		for r in 0..row {
			for c in 0..col {
				new_vec.push(self.data[self.linear_index(c, r) as usize]);
			}
			for c in 0..col {
				new_vec.push(id.data[id.linear_index(c, r) as usize].into());
			}
		}
		Matrix { data: new_vec, col: 2 * col, row }
	}

	fn extract_augment(self) -> Matrix<K> {
		let (col, row, _) = self.get_shape();
		let mut new_vec = Vec::with_capacity(row.pow(2) as usize);
		for r in 0..row {
			for c in 0..col {
				if c >= col / 2 {
					new_vec.push(self.data[self.linear_index(c, r) as usize]);
				}
			}
		}
		Matrix { data: new_vec, col: col / 2, row }
	}
}

impl<K: Default + Copy + One + PartialEq + Add<Output = K> + Mul<Output = K> + Div<Output = K> + Sub<Output = K> + Neg<Output = K> + fmt::Display, const R: usize>  matrix<K, R, R> {
	pub fn inverse(&self) -> Option<Self> {
		if R == 0 {
			Some(*self);
		}
		if self.determinant() == K::default() {
			return None;
		}
		let mut result = *self;
		for r in 0..R {
			for c in 0..R {
				if r == c {
					result.0[r][c] = K::one();
				}
				else {
					result.0[r][c] = K::default();
				}
			}
		}
		let mut copy = *self;
		let mut r = 0;	
		for c in 0..R {
			if let Some(row1) = copy.find_best_first_row(r, c) {
				if row1 != r {
					copy.row_swap(row1, r);
					result.row_swap(row1, r);
				}
				println!("{}, {}", result, copy);
				result.0[r] = (vector::from(result.0[r]) * (K::one() / copy.0[r][c])).0;
				copy.0[r] = (vector::from(copy.0[r]) * (K::one() / copy.0[r][c])).0;
				println!("{}, {}", result, copy);
				for r2 in 0..R {
					if r2 != r {
						result.0[r2] = (vector::from(result.0[r2]) - vector::from(result.0[r]) * copy.0[r2][c]).0;
						copy.0[r2] = (vector::from(copy.0[r2]) - vector::from(copy.0[r]) * copy.0[r2][c]).0;
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
		Some(result)
	}
}