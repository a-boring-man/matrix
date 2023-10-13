use std::{ops::{AddAssign, Mul, Add}, fmt::Display};

use crate::matrix::basic_definition::{trait_definition::{Complexe}, definition::{vector, matrix}};

impl<K: Copy + Default + Display + Complexe + Add<Output = K> + Mul<Output = K>, const R: usize, const C: usize> Mul<vector<K, C>> for matrix<K, R, C> {
	type Output = vector<K, R>;

	fn mul(self, rhs: vector<K, C>) -> Self::Output {
		let mut result = vector::<K, R>::new();
		for r in 0..R {
			result.0[r] = vector(self.0[r]).dot(rhs);
		}
		result
	}
}

impl<K: Copy + Default + Display + Complexe + Add<Output = K> + Mul<Output = K>, const R: usize, const C: usize, const C2: usize> Mul<matrix<K, C, C2>> for matrix<K, R, C> {
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