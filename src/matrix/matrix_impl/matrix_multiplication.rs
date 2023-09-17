use std::ops::{AddAssign, Mul, Add};

use crate::matrix::basic_definition::{trait_definition::{Scalar, Complexe}, definition::{Matrix, Vector, vector, matrix}};

impl<K: Scalar + Default + AddAssign> Matrix<K> {
	pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
		let (nbr_col, nbr_row, _) = self.get_shape();
		if vec.v.len() != nbr_col as usize || vec.v.len() == 0 {
			panic!("dimension error during matrix-vector multiplication")
		}
		let mut tmp_vec = Vec::with_capacity(nbr_col as usize);
		for r in 0..nbr_row {
			let mut tmp_val = K::default();
			for c in 0..nbr_col {
				tmp_val += vec.v[c as usize] * self.data[self.linear_index(c, r) as usize];
			}
			tmp_vec.push(tmp_val);
		}
		Vector { v: tmp_vec }
	}

	pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
		let (nbr_col1, nbr_row1, _) = self.get_shape();
		let (nbr_col2, nbr_row2, _) = mat.get_shape();
		if nbr_col1 != nbr_row2 || nbr_col1 == 0 {
			panic!("dimension error during matrix-matrix multiplication")
		}
		let mut tmp_vec = Vec::with_capacity(nbr_col1 as usize * nbr_row2 as usize);
		for r1 in 0..nbr_row1 {
			for c2 in 0..nbr_col2 {
				let mut tmp_val = K::default();
				for r2 in 0..nbr_row2 {
					tmp_val += self.data[self.linear_index(r2, r1) as usize] * mat.data[mat.linear_index(c2, r2) as usize];
				}
				tmp_vec.push(tmp_val);
			}
		}
		Matrix {data: tmp_vec, col: nbr_col2, row: nbr_row1}
	}
}

impl<K: Copy + Default + Complexe + Add<Output = K> + Mul<Output = K>, const R: usize, const C: usize> Mul<vector<K, C>> for matrix<K, R, C> {
	type Output = vector<K, R>;

	fn mul(self, rhs: vector<K, C>) -> Self::Output {
		let mut result = vector::<K, R>::new();
		for r in 0..R {
			result.0[r] = vector(self.0[r]).dot(rhs);
		}
		result
	}
}

impl<K: Copy + Default + Complexe + Add<Output = K> + Mul<Output = K>, const R: usize, const C: usize, const C2: usize> Mul<matrix<K, C, C2>> for matrix<K, R, C> {
	type Output = matrix<K, R, C2>;

	fn mul(self, rhs: matrix<K, C, C2>) -> Self::Output {
		let mut result = matrix::<K, R, C2>::new();
		for r in 0..R {
			for c2 in 0..C2 {
				result.0[r][c2] = vector(self.0[r]).dot(rhs.isolate_column_vector(c2));
			}
		}
		result
	}
}