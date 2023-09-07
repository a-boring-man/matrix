use std::ops::Mul;

use num_traits::One;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix, error::MatrixError};

impl<K: Scalar + Default + std::cmp::PartialEq + std::convert::From<i32> + One + for<'a> std::ops::SubAssign<&'a K> + for<'a> std::ops::Div<&'a K, Output = K> + for <'a> std::ops::MulAssign<&'a K> + for<'a> std::ops::AddAssign<&'a K>> Matrix<K> where for <'a> &'a K: Mul<&'a K, Output = K> {
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