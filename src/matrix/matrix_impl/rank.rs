use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Matrix, matrix}};
use std::{ops::{Sub, Div}, fmt::Display};

use num_traits::identities::One;

use std::ops::Mul;

impl<K: Scalar + Default + One + for<'a> std::ops::SubAssign<&'a K> + for<'a> std::ops::Div<&'a K, Output = K> + for <'a> std::ops::MulAssign<&'a K> + for<'a> std::ops::AddAssign<&'a K> + std::cmp::PartialEq> Matrix<K> where for <'a> &'a K: Mul<&'a K, Output = K> {
	pub fn rank(&self) -> u32 {
		let reduced = self.row_echelon();

		let mut rank = 0;
		for r in 0..reduced.row {
			for c in 0..reduced.col {
				if reduced.data[reduced.linear_index(c, r) as usize] != K::default() {
					rank += 1;
					break;
				}
			}
		}
		rank
	}
}

impl<K: Copy + Default + One + PartialEq + Display + Div<Output = K> + Sub<Output = K>, const R: usize, const C: usize> matrix<K, R, C> {
	pub fn rank(&self) -> u32 {
		let reduced = self.row_echelon();
		let mut rank = 0;
		if R == 0 || C == 0 {
			return rank;	
		}
		let mut r = 0;		
		for c in 0..C {
			if reduced.0[r][c] != K::default() {
				println!("elem{}", reduced.0[r][c]);
				r += 1;
				rank += 1;
			}
			if r == R {break;}
		}
		rank
	}
}