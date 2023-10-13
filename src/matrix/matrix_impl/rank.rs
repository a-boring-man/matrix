use crate::matrix::basic_definition::{trait_definition::{Scalar, Zero}, definition::{matrix}};
use std::{ops::{Sub, Div}, fmt::{Display, Debug}};

use num_traits::identities::One;

use std::ops::Mul;

impl<K: Copy + Default + Debug + Display + One + Zero + PartialEq + Div<Output = K> + Sub<Output = K>, const R: usize, const C: usize> matrix<K, R, C> {
	pub fn rank(&self) -> u32 {
		let reduced = self.row_echelon();
		//println!("echelon {}", reduced);
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