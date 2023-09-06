use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix};

impl<K: Scalar> Matrix<K> {
	pub fn transpose(&self) -> Matrix<K> {
		if self.data.len() == 0 {
			panic!("wrong dimension matrix transpose");
		}
		let (nbr_col, nbr_row, _) = self.get_shape();
		let mut result: Vec<K> = Vec::with_capacity(nbr_col as usize * nbr_row as usize);
		for c in 0..nbr_col {
			for r in 0..nbr_row {
				result.push(self.data[self.linear_index(c, r) as usize].clone());
			}
		}
		Matrix {
			data: result,
			col: nbr_row, 
			row: nbr_col
		}
	}
}