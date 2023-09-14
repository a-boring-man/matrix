use crate::matrix::basic_definition::{trait_definition::{Scalar, Normable}, definition::{Vector, vector}};

// impl<K: Normable, const L: usize> vector<K, L> {
// 	pub fn norm_taxicab(&self) -> K {

// 	}
// }

impl<K: Scalar + Normable> Vector<K> {
	pub fn norm_taxicab(&self) -> K {
		if self.v.len() <= 0 {
			panic!("vector len was zero in norm calculation");
		}
		self.v.iter().map(|a| a.norm() ).sum()
	}

	pub fn norm_euclidean(&self) -> K {
		if self.v.len() <= 0 {
			panic!("vector len was zero in norm calculation");
		}
		let tmp: K = self.v.iter().map(|a| a.norm().square() ).sum();
		tmp.square_root()
	}

	pub fn norm_supremum(&self) -> K {
		if self.v.len() <= 0 {
			panic!("vector len was zero in norm calculation");
		}
		let mut big = self.v[0].clone();
		let mut smole = self.v[0].clone();
		for number in &self.v {
			if number > &big {
				big = number.clone();
			}
			if number < &smole {
				smole = number.clone();
			}
		}
		if big.norm() >= smole.norm(){
			return big.clone().norm();
		}
		smole.clone().norm()
	}
}