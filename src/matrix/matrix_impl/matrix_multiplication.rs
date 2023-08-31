use std::ops::{AddAssign, Mul};

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Matrix, Vector}};

impl<K: Scalar + Default + AddAssign> Matrix<K> where for<'a> &'a K: Mul<&'a K, Output = K> {
	pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
		let (nbr_col, nbr_row) = self.get_shape();
		if vec.v.len() != nbr_col as usize || vec.v.len() == 0 {
			panic!("dimension error during matrix-vector multiplication")
		}
		let mut tmp_vec = Vec::with_capacity(nbr_col as usize);
		for r in 0..nbr_row {
			let mut tmp_val = K::default();
			for c in 0..nbr_col {
				tmp_val += &vec.v[c as usize] * &self.data[self.linear_index(r, c) as usize];
			}
			tmp_vec.push(tmp_val);
		}
		Vector { v: tmp_vec }
	}

	pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
		let (nbr_col1, nbr_row1) = self.get_shape();
		let (nbr_col2, nbr_row2) = mat.get_shape();
		if nbr_col1 != nbr_row2 || nbr_col1 == 0 {
			panic!("dimension error during matrix-matrix multiplication")
		}
		let mut tmp_vec = Vec::with_capacity(nbr_col1 as usize * nbr_row2 as usize);
		for r1 in 0..nbr_row1 {
			for c2 in 0..nbr_col2 {
				let mut tmp_val = K::default();
				for r2 in 0..nbr_row2 {
					tmp_val += &self.data[self.linear_index(r1, r2) as usize] * &mat.data[mat.linear_index(r2, c2) as usize];
				}
				tmp_vec.push(tmp_val);
			}
		}
		Matrix::from((tmp_vec, nbr_col2, nbr_row1))
	}
}