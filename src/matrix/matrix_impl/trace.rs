use std::{ops::{AddAssign, Add}, fmt::Display};
use num_traits::identities::One;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{matrix}};

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