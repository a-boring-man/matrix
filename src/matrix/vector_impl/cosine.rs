use std::{ops::Add, ops::{Mul, Div}};

use crate::matrix::basic_definition::{trait_definition::{Scalar, Normable, Complexe}, definition::{Vector, vector}};

impl<K: Scalar + Normable + std::ops::Div<Output = K>> Vector<K> {
	pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K {
		u.dot(v) / (u.norm_euclidean() * v.norm_euclidean())
	}
}

impl<K: Scalar + Normable + Complexe + std::ops::Div<Output = K> + std::iter::Sum<<K as std::ops::Mul>::Output>> Vector<K> {
	pub fn angle_cos_complex(u: &Vector<K>, v: &Vector<K>) -> K {
		u.dot_complex(v) / (u.norm_euclidean() * v.norm_euclidean())
	}
}

fn _angle_cos<K: Complexe + Normable + Default + Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K>, const L: usize> (v1: vector<K, L>, v2: vector<K, L>) -> K {
	v1.dot(v2) / (v1.norm_euclidean() * v2.norm_euclidean())
}