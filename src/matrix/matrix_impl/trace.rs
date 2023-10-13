use std::ops::Add;
use num_traits::identities::One;

use crate::matrix::basic_definition::definition::Matrix;

/// return type default value if matrix is of 0th dimension
impl<K: Copy + One + Default + Add<Output = K>, const R: usize> Matrix<K, R, R> {
	pub fn trace(&self) -> K {
		match R {
			0 => {K::default()}
			_ => {
				self.0.iter().enumerate().fold(K::default(), |acc, (i, vec)| {
					acc + vec[i]
				})
			}
		}
	}
	pub fn tracex(&self) -> K {
		match R {
			0 => {K::default()}
			_ => {
				self.0.iter().enumerate().fold(K::one(), |acc, (i, vec)| {
					acc * vec[i]
				})
			}
		}
	}
}