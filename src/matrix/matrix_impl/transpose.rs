use crate::matrix::basic_definition::definition::Matrix;

impl<K: Copy + Default, const R: usize, const C: usize> Matrix<K, R, C> {
	
	/// return the transposed matrix of self
	pub fn transpose(&self) -> Matrix<K, C, R> {
		let mut result = [[K::default(); R]; C];
		for (c, col) in result.iter_mut().enumerate().take(C) {
			for (r, row) in col.iter_mut().enumerate().take(R) {
				*row = self.0[r][c];
			}
		}
		Matrix(result)
	}
}