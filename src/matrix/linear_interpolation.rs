use std::{fmt::Display, ops::{Sub, Mul, Add}};

use super::basic_definition::{trait_definition::Scalar, definition::{Matrix, Vector}};

impl<K: Scalar + Display> Matrix<K> {
	pub fn lerp(m1: &Matrix<K>, m2: &Matrix<K>, r: K) -> Matrix<K> {
		m1.add(&m2.sub(m1).scale(r))
	}
}

impl<K: Scalar + Display> Vector<K> {
	pub fn lerp(v1: &Vector<K>, v2: &Vector<K>, r: &K) -> Vector<K> {
		v1.add(&v2.sub(v1).scale(r))
	}
}

pub fn _lerp<V: Sub<Output = V> + Mul<K, Output = V> + Add<Output = V> + Clone, K: Scalar>(o1: V, o2: V, p: K) -> V {
	o1.clone() + (o2 - o1) * p
}