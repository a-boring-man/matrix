use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix, error::MatrixError};

impl<K: Scalar> Matrix<K> {
	pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
		let det = self.determinant()?;
		if det == 0 {
			return Err(MatrixError::NotInversible);
		}
		
	}
}