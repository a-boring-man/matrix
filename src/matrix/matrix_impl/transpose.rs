use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{matrix}};

impl<K: Copy + Default, const R: usize, const C: usize> matrix<K, R, C> {
	pub fn transpose(&self) -> matrix<K, C, R> {
		let mut result = [[K::default(); R]; C];
		for c in 0..C {
			for r in 0..R {
				result[c][r] = self.0[r][c];
			}
		}
		matrix(result)
	}
}