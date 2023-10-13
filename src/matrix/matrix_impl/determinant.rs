use crate::matrix::basic_definition::definition::{Matrix, Vector};
use num_traits::identities::One;
use std::ops::{Mul, Div, Sub, Neg, Add};

impl<K: Default + Copy + One + PartialEq + Mul<Output = K> + Add<Output = K> + Div<Output = K> + Sub<Output = K> + Neg<Output = K>, const R: usize> Matrix<K, R, R> {

	pub fn determinant(&self) -> K {
		let find_first_row = |m: &Matrix<K, R, R>, row: usize, col: usize| -> Option<usize> {
			for (i, vec) in m.0.iter().enumerate().skip(row) {
				if vec[col] != K::default() {
					return Some(i);
				}
			}
			None
		};

		let row_swap = |m: &mut Matrix<K, R, R>, r1: usize, r2: usize| {
			for c in 0..R {
				let tmp: K = m.0[r1][c];
				m.0[r1][c] = m.0[r2][c];
				m.0[r2][c] = tmp;
			}
		};

		let mut copy = *self;
		let mut r = 0;
		let mut swap = false;
		for c in 0..R {
			if let Some(row1) = find_first_row(&copy, r, c) {
				if row1 != r {
					row_swap(&mut copy, row1, r);
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
		//println!("copy {}", copy);
		let mut det = copy.tracex();
		//println!("tracex {}", det);
		if swap {
			det = -det;
		}
		det
	}
}