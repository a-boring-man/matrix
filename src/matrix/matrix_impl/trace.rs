use std::{ops::{AddAssign, Add}, fmt::Display};
use num_traits::identities::One;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Matrix, matrix}};

impl<K: Scalar + Default + AddAssign> Matrix<K> {
	pub fn trace(&self) -> K {
		let (nbr_col, _, _) = self.get_shape();
		if !self.is_square() || self.get_shape().0 == 0 {
			panic!("matrix is not squared, couldn't compute trace");
		}
		let mut result = K::default();
		for i in 0..nbr_col {
			let index = i * nbr_col + i;
			result += self.data[index as usize].clone();
		}
		result
	}
}

/// return type default value if matrix is of 0th dimension
impl<K: Copy + One + Display + Default + Add<Output = K>, const R: usize> matrix<K, R, R> {
	pub fn trace(&self) -> K {
		match R {
			0 => {return K::default();}
			_ => {
				self.0.iter().enumerate().fold(K::default(), |acc, (i, vec)| {
					acc + vec[i]
				})
			}
		}
	}
	pub fn tracex(&self) -> K {
		match R {
			0 => {return K::default();}
			_ => {
				self.0.iter().enumerate().fold(K::one(), |acc, (i, vec)| {
					//println!("av acc {}", acc);
					let tmp = acc * vec[i];
					//println!("ap acc {}", acc);
					tmp
				})
			}
		}
	}
}