use std::ops::{Mul, Sub};

use crate::matrix::basic_definition::definition::Vector;

pub fn _cross_product
	<K: Copy + Mul<Output = K> + Sub<Output = K>>
	(v1: Vector<K, 3>, v2: Vector<K, 3>) -> Vector<K, 3> {
	Vector([
		v1.0[1] * v2.0[2] - v1.0[2] * v2.0[1],
        v1.0[2] * v2.0[0] - v1.0[0] * v2.0[2],
        v1.0[0] * v2.0[1] - v1.0[1] * v2.0[0],
	])
}