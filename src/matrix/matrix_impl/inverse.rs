use num_traits::One;
use std::ops::{Neg, Add, Sub, Mul, Div};
use crate::matrix::basic_definition::{trait_definition::Zero, definition::{Matrix, Vector}};

impl<K: Copy + Default + One + Zero + PartialEq + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + Neg<Output = K>, const R: usize>  Matrix<K, R, R> {
	pub fn inverse(&self) -> Option<Self> {
		if self.determinant() == K::default() {
			return None;
		}
		let mut result = Matrix::identity();
		let mut copy = *self;
		let mut r = 0;	
		for c in 0..R {
			if let Some(row1) = copy.find_best_first_row(r, c) {
				if row1 != r {
					copy.row_swap(row1, r);
					result.row_swap(row1, r);
				}
				result.0[r] = (Vector::from(result.0[r]) * (K::one() / copy.0[r][c])).0;
				copy.0[r] = (Vector::from(copy.0[r]) * (K::one() / copy.0[r][c])).0;
				for r2 in 0..R {
					if r2 != r {
						result.0[r2] = (Vector::from(result.0[r2]) - Vector::from(result.0[r]) * copy.0[r2][c]).0;
						copy.0[r2] = (Vector::from(copy.0[r2]) - Vector::from(copy.0[r]) * copy.0[r2][c]).0;
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