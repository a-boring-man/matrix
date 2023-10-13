use core::fmt;
use std::{ops::{Div, Sub}, fmt::{Display, Debug}};
use num_traits::identities::One;

use crate::matrix::basic_definition::{trait_definition::{Scalar, Zero}, definition::{matrix, vector}};

impl<K: Copy + Default + Debug + Display + Zero + One + PartialEq + Div<Output = K> + Sub<Output = K>, const R: usize, const C: usize> matrix<K, R, C> {
	pub (crate) fn find_best_first_row(&self, row: usize, col: usize) -> Option<usize> {
		let zero = K::default();
		for (i, vec) in self.0.iter().enumerate().skip(row) {
			if !vec[col].close_to_zero() {
				//println!("correct line found : {}", vec[col]);
				return Some(i);
			}
		}
		None
	}

	pub(crate) fn row_swap(&mut self, row1: usize, row2: usize) {
		for c in 0..C {
			let tmp = self.0[row2][c];
			self.0[row2][c] = self.0[row1][c];
			self.0[row1][c] = tmp;
		}
	}

	pub fn row_echelon(&self) -> Self {
		if C == 0 || R == 0 {
			return *self;
		}
		let mut result = *self;

		let mut r = 0;	
		for c in 0..C {
			//println!("number of column treted {}", c);
			if let Some(row1) = result.find_best_first_row(r, c) {
				if row1 != r {
					result.row_swap(row1, r);
				}
				result.0[r] = (vector::from(result.0[r]) * (K::one() / result.0[r][c])).0;
				//println!("nullifing the first column {:?}", result.0[r]);
				for r2 in 0..R {
					if r2 != r {
						result.0[r2] = (vector::from(result.0[r2]) - vector::from(result.0[r]) * result.0[r2][c]).0;
						//println!("nulifiying other row {:?}", result.0[r2]);
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
		result
	}
}