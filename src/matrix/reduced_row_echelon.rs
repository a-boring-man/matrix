use super::definition::{Scalar, Matrix};
use std::ops::BitXor;

impl<K: Scalar + Default + std::ops::MulAssign + std::ops::AddAssign> Matrix<K> where for<'a> &'a K: BitXor<&'a K, Output = &'a K> {

	fn row_swap(&mut self, r1: usize, r2: usize) {
		let (nbr_col, _) = self.get_shape();
		for c in 0..nbr_col {
			let mut a: &K = &self.data[self.linear_index(r1 as u8, c) as usize];
			let mut b: &K = &self.data[self.linear_index(r2 as u8, c) as usize];
			a = a ^ b;
			b = a ^ b;
			a = a ^ b;
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
			result.push(self.data[self.linear_index(r as u8, c) as usize] * s);
		}
		result
	}

	fn row_addition(&mut self, r: usize, vec: Vec<K>) {
		let (nbr_col, _) = self.get_shape();
		for c in 0..nbr_col {
			self.data[self.linear_index(r as u8, c) as usize] += vec[c];
		}
	}

	pub fn row_echelon(&self) -> Self {
		let (nbr_col, nbr_row) = self.get_shape();
		let lead_row: usize = 0;

		for r in 0..nbr_row {
			let r2 = lead_row;
			while r2 < nbr_row && self.data[self.linear_index(r2, lead_row)] == K::default() {
				r2 += 1;
			}
			
		}
	}
}