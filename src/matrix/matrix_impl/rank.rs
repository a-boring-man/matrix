use crate::matrix::basic_definition::{trait_definition::Zero, definition::Matrix};
use std::ops::{Sub, Div};

use num_traits::identities::One;

impl<K: Copy + Default + One + Zero + PartialEq + Div<Output = K> + Sub<Output = K>, const R: usize, const C: usize> Matrix<K, R, C> {
	pub fn rank(&self) -> u32 {
		let reduced = self.row_echelon();
		let mut rank = 0;
		if R == 0 || C == 0 {
			return rank;	
		}
		let mut r = 0;		
		for c in 0..C {
			if !reduced.0[r][c].close_to_zero() {
				r += 1;
				rank += 1;
			}
			if r == R {break;}
		}
		rank
	}
}