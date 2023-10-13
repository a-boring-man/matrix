use std::ops::{Mul, Add};

use crate::matrix::basic_definition::definition::{Vector, Matrix};

impl<K: Copy + Default + Add<Output = K> + Mul<Output = K>, const R: usize, const C: usize> Mul<Vector<K, C>> for Matrix<K, R, C> {
	type Output = Vector<K, R>;

	fn mul(self, rhs: Vector<K, C>) -> Self::Output {
		let mut result = Vector::<K, R>::new();
		for r in 0..R {
			result.0[r] = Vector(self.0[r]).dot(rhs);
		}
		result
	}
}

impl<K: Copy + Default + Add<Output = K> + Mul<Output = K>, const R: usize, const C: usize, const C2: usize> Mul<Matrix<K, C, C2>> for Matrix<K, R, C> {
	type Output = Matrix<K, R, C2>;

	fn mul(self, rhs: Matrix<K, C, C2>) -> Self::Output {
		let mut result = Matrix::<K, R, C2>::new();
		for r in 0..R {
			for c2 in 0..C2 {
				result.0[r][c2] = Vector(self.0[r]).dot(rhs.isolate_column_vector(c2));
			}
		}
		result
	}
}