use crate::matrix::basic_definition::{definition::{Matrix, Vector}, trait_definition::Zero};
use num_traits::identities::One;
use std::ops::{Mul, Div, Sub, Neg, Add};

impl<K: Default + Copy + One + Zero + PartialEq + Mul<Output = K> + Add<Output = K> + Div<Output = K> + Sub<Output = K> + Neg<Output = K>, const R: usize> Matrix<K, R, R> {

	pub fn determinant(&self) -> K {
		let mut copy = *self;
		let mut r = 0;
		let mut swap = false;
		for c in 0..R {
			if let Some(row1) = copy.find_best_first_row(r, c) {
				if row1 != r {
					copy.row_swap(row1, r);
					swap = !swap;
				}
				for r2 in r..R {
					if r2 != r {
						copy.0[r2] = (Vector::from(copy.0[r2]) - Vector::from(copy.0[r]) * (copy.0[r2][c] / copy.0[r][c])).0;
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
		let mut det = copy.tracex();
		if swap {
			det = -det;
		}
		det
	}
}