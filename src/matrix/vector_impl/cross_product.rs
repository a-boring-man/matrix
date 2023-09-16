use std::ops::{Add, Mul, Sub};

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Vector, vector}};

impl<K: Scalar> Vector<K> {
	pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
		if u.v.len() != 3 || v.v.len() != 3 {
			panic!("cross product call on a non 3d vector");
		}
		Vector {
			v: vec![u.v[1] * v.v[2] - u.v[2] * v.v[1], u.v[2] * v.v[0] - u.v[0] * v.v[2], u.v[0] * v.v[1] - u.v[1] * v.v[0]]
		}
	}
}

pub fn _cross_product<K: Copy + Mul<Output = K> + Sub<Output = K>> (v1: vector<K, 3>, v2: vector<K, 3>) -> vector<K, 3> {
	vector([
		v1.0[1] * v2.0[2] - v1.0[2] * v2.0[1],
        v1.0[2] * v2.0[0] - v1.0[0] * v2.0[2],
        v1.0[0] * v2.0[1] - v1.0[1] * v2.0[0],
	])
}