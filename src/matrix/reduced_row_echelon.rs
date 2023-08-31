use super::{definition::Matrix, trait_definition::Scalar};
use num_traits::identities::One;
use std::ops::Mul;

impl<K: Scalar + Default + One + for<'a> std::ops::SubAssign<&'a K> + for<'a> std::ops::Div<&'a K, Output = K> + for <'a> std::ops::MulAssign<&'a K> + for<'a> std::ops::AddAssign<&'a K> + std::cmp::PartialEq> Matrix<K> where for <'a> &'a K: Mul<&'a K, Output = K> {

	fn row_swap(&mut self, r1: usize, r2: usize) {
		let (nbr_col, _) = self.get_shape();
		for c in 0..nbr_col {
			let swap = self.data[self.linear_index(r1 as u8, c) as usize].clone();
			let tmp = self.data[self.linear_index(r2 as u8, c) as usize].clone();
			let i1 = self.linear_index(r1 as u8, c) as usize;
			let i2 = self.linear_index(r2 as u8, c) as usize;
			self.data[i1] = tmp;
			self.data[i2] = swap;
		}
	}

	fn row_self_scale_mul(&mut self, r: usize, s: &K) {
		let (nbr_col, _) = self.get_shape();
		for c in 0..nbr_col {
			let i1 = self.linear_index(r as u8, c) as usize;
			self.data[i1] *= s;
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
			let i1 = self.linear_index(r as u8, c) as usize;
			self.data[i1] -= &vec[c as usize];
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
				// reduce the pivot to one
				let mut tmp = result.clone();
				tmp.row_self_scale_mul(r.into(), &s);
				result = tmp;
			}
			// nullifingm all the other row
			for i in 0..nbr_row {
				if i != r {
					let s = result.data[result.linear_index(i, lead) as usize].clone();
					let scaled_row = result.row_scale_mul(r as usize, s);
					result.row_addition(i as usize, scaled_row);
				}
			}
		}
		result
	}
}