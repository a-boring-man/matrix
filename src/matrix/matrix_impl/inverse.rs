use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix, error::MatrixError};

impl<K: Scalar + Default + std::cmp::PartialEq, T: Scalar> Matrix<K> {
	pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
		let det = self.determinant()?;
		if det == K::default() {
			return Err(MatrixError::NotInversible);
		}
		let result = self.clone();
		result.augment_matrix(Matrix::identity(self.col));
	}

	pub fn augment_matrix(&mut self, matrix: Matrix<T>) {
		let dimension = self.get_shape();
		for r in 0..dimension.1 {
			for c in 0..dimension.0 {
				
			}
		}
	}
}