use std::ops::{Add, Mul, Sub};

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{vector}};

pub fn _cross_product
	<K: Copy + Mul<Output = K> + Sub<Output = K>>
	(v1: vector<K, 3>, v2: vector<K, 3>) -> vector<K, 3> {
	vector([
		v1.0[1] * v2.0[2] - v1.0[2] * v2.0[1],
        v1.0[2] * v2.0[0] - v1.0[0] * v2.0[2],
        v1.0[0] * v2.0[1] - v1.0[1] * v2.0[0],
	])
}