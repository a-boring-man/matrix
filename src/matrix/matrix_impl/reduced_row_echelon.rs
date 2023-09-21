use num_traits::identities::One;

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Matrix, matrix}};

impl<K: Scalar + Default + One> Matrix<K> {

	fn row_swap(&mut self, r1: usize, r2: usize) {
		let (nbr_col, _, _) = self.get_shape();
		for c in 0..nbr_col {
			let swap = self.data[self.linear_index(c, r1 as u8) as usize].clone();
			let tmp = self.data[self.linear_index(c, r2 as u8) as usize].clone();
			let i1 = self.linear_index(c, r1 as u8) as usize;
			let i2 = self.linear_index(c, r2 as u8) as usize;
			self.data[i1] = tmp;
			self.data[i2] = swap;
		}
	}

	fn row_self_scale_mul(&mut self, r: usize, s: K) {
		let (nbr_col, _, _) = self.get_shape();
		for c in 0..nbr_col {
			let i1 = self.linear_index(c, r as u8) as usize;
			self.data[i1] = self.data[i1] * s;
		}
	}

	fn row_scale_mul(&self, r: usize, s: K) -> Vec<K>{
		let (nbr_col, _, _) = self.get_shape();
		let mut result: Vec<K> = Vec::with_capacity(nbr_col as usize);
		for c in 0..nbr_col {
			result.push(self.data[self.linear_index(c, r as u8) as usize] * s);
		}
		result
	}

	fn row_addition(&mut self, r: usize, vec: Vec<K>) {
		let (nbr_col, _, _) = self.get_shape();
		for c in 0..nbr_col {
			let i1 = self.linear_index(c, r as u8) as usize;
			self.data[i1] = self.data[i1] - vec[c as usize];
		}
	}

	pub fn row_echelon(&self) -> Self {
		if self.data.len() == 0 {
			panic!("zero dimension matrix RREF");
		}
		let mut result = self.clone();
		let (nbr_col, nbr_row, _) = result.get_shape();
		let mut lead = 0;

		for r in 0..nbr_row {
			// if at the end exit
			if lead >= nbr_col {
				break;
			}

			let mut i = r;

			// skip row with 0
			while result.data[result.linear_index(lead, i as u8) as usize] == K::default() {
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
			if result.data[result.linear_index(lead, r as u8) as usize] != K::default() {
				// the scalare by witch multiply the first row
				let s: K = K::one() / result.data[result.linear_index(lead, r as u8) as usize];
				// reduce the pivot to one
				let mut tmp = result.clone();
				tmp.row_self_scale_mul(r.into(), s);
				result = tmp;
			}
			// nullifingm all the other row
			for i in 0..nbr_row {
				if i != r {
					let s = result.data[result.linear_index(lead, i) as usize].clone();
					let scaled_row = result.row_scale_mul(r as usize, s);
					result.row_addition(i as usize, scaled_row);
				}
			}
		}
		result
	}
}

impl<K: Copy + Default + One + PartialEq, const R: usize, const C: usize> matrix<K, R, C> {
	fn find_best_first_row(&self, row:usize, col: usize) -> Option<usize> {
		if col >= C || row >= R {
			panic!("wrong column or row number in search first row")
		}
		let max = K::default();
		for (i, vec) in self.0.iter().enumerate().skip(row) {
			if vec[col] != max {
				return Some(i);
			}
		}
		None
	}

	fn row_swap(&mut self, row1: usize, row2: usize) {
		for c in 0..C {
			let tmp = self.0[row2][c];
			self.0[row2][c] = self.0[row1][c];
			self.0[row1][c] = tmp;
		}
	}
}