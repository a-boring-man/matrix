use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix, error::MatrixError};

impl<K: Scalar> Matrix<K> {
	pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
		if self.len() == 1 || !self.
	}
}