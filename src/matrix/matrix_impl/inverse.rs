use core::fmt;
use std::{ops::{Sub, Mul, Div, Add, Neg}, fmt::Debug};
use num_traits::One;

use crate::matrix::basic_definition::{trait_definition::{Scalar, Zero}, definition::{matrix, vector}};

impl<K: Default + Copy + Debug + One + Zero + PartialEq + Add<Output = K> + Mul<Output = K> + Div<Output = K> + Sub<Output = K> + Neg<Output = K> + fmt::Display, const R: usize>  matrix<K, R, R> {
	pub fn inverse(&self) -> Option<Self> {
		if R == 0 {
			Some(*self);
		}
		if self.determinant() == K::default() {
			return None;
		}
		let mut result = matrix::identity();
		let mut copy = *self;
		let mut r = 0;	
		for c in 0..R {
			if let Some(row1) = copy.find_best_first_row(r, c) {
				if row1 != r {
					copy.row_swap(row1, r);
					result.row_swap(row1, r);
				}
				println!("{}, {}", result, copy);
				result.0[r] = (vector::from(result.0[r]) * (K::one() / copy.0[r][c])).0;
				copy.0[r] = (vector::from(copy.0[r]) * (K::one() / copy.0[r][c])).0;
				println!("{}, {}", result, copy);
				for r2 in 0..R {
					if r2 != r {
						result.0[r2] = (vector::from(result.0[r2]) - vector::from(result.0[r]) * copy.0[r2][c]).0;
						copy.0[r2] = (vector::from(copy.0[r2]) - vector::from(copy.0[r]) * copy.0[r2][c]).0;
					}
					else {
						continue;
					}
				}	
				r += 1;
			}
			else {
				continue;
			}
		}
		Some(result)
	}
}