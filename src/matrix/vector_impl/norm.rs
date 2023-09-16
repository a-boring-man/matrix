use crate::matrix::basic_definition::{trait_definition::{Scalar, Normable}, definition::{Vector, vector}};

use std::ops::Add;

impl<K: Normable + Default + Copy + Add<Output = K>, const L: usize> vector<K, L> {
	/// Norme 1 return 0 if vector is empty
	pub fn norm_taxicab(&self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			result = result + self.0[i].norm();
		}
		result
	}

	/// Norm 2 return 0 if the vector is empty
	pub fn norm_euclidean(&self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			result = result + self.0[i].norm().square();
		}
		result.square_root()
	}

	pub fn norm_supremum(&self) -> K {
		let mut result: K = K::default();
		for i in 0..L {
			match self.0[i].norm() > result {
				true => result = self.0[i].norm(),
				false => {},
			}
		}
		result
	}
}

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