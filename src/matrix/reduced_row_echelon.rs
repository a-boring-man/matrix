use super::definition::{Scalar, Matrix};
use num_traits::identities::One;
use std::ops::Mul;

impl<K: Scalar + Default + One + for<'a> std::ops::Div<&'a K, Output = K> + std::ops::MulAssign + std::ops::AddAssign + std::cmp::PartialEq> Matrix<K> where for <'a> &'a K: Mul<&'a K, Output = K> {

	fn row_swap(&mut self, r1: usize, r2: usize) {
		let (nbr_col, _) = self.get_shape();
		for c in 0..nbr_col {
			let swap = self.data[self.linear_index(r1 as u8, c) as usize].clone();
			self.data[self.linear_index(r1 as u8, c) as usize] = self.data[self.linear_index(r2 as u8, c) as usize];
			self.data[self.linear_index(r2 as u8, c) as usize] = swap;
		}
	}

	fn row_self_scale_mul(&mut self, r: usize, s: K) {
		let (nbr_col, _) = self.get_shape();
		for c in 0..nbr_col {
			self.data[self.linear_index(r as u8, c) as usize] *= s;
		}
	}

	fn row_scale_mul(& self, r: usize, s: K) -> Vec<K>{
		let (nbr_col, _) = self.get_shape();
		let mut result: Vec<K> = Vec::with_capacity(nbr_col as usize);
		for c in 0..nbr_col {
			result.push(&self.data[self.linear_index(r as u8, c) as usize] * &s);
		}
		result
	}

	fn row_addition(&mut self, r: usize, vec: Vec<K>) {
		let (nbr_col, _) = self.get_shape();
		for c in 0..nbr_col {
			self.data[self.linear_index(r as u8, c) as usize] += vec[c as usize];
		}
	}

	pub fn row_echelon(&self) -> Self {
		if self.data.len() == 0 {
			panic!("zero dimension matrix RREF");
		}
		let mut result = self.clone();
		let (nbr_col, nbr_row) = result.get_shape();
		let mut lead = 0;

		for r in 0..nbr_row {
			// if at the end exit
			if lead >= nbr_col {
				break;
			}

			let mut i = r;

			// skip row with 0
			while result.data[result.linear_index(i as u8, lead) as usize] == K::default() {
				i += 1;

				// if no row where found
				if nbr_row == i {
					// so to swap the first row with the first raw
					i = r;
					lead += 1;

					// exit if last column
					if lead == nbr_col {
						return result;
					}
				}
			}

			result.row_swap(r as usize, i as usize);

			// if i find something
			if result.data[result.linear_index(r as u8, lead) as usize] != K::default() {
				// the scalare by witch multiply the first row
				let s: K = K::one() / &result.data[result.linear_index(r as u8, lead) as usize];
				result.row_self_scale_mul(r.into(), s);
			}
		}
		result
	}
}