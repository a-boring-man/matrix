use super::{definition::Matrix, trait_definition::Scalar};

impl<K: Scalar> Matrix<K> {
	pub fn transpose(&self) -> Matrix<K> {
		if self.data.len() == 0 {
			panic!("wrong dimension matrix transpose");
		}
		let (nbr_col, nbr_row) = self.get_shape();
		let mut result: Vec<K> = Vec::with_capacity(nbr_col as usize * nbr_row as usize);
		for c in 0..nbr_col {
			for r in 0..nbr_row {
				result.push(self.data[self.linear_index(r, c) as usize].clone());
			}
		}
		Matrix::from((result, nbr_row, nbr_col))
	}
}