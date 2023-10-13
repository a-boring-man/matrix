use std::{fmt::Display, ops::{Sub, Mul, Add}};

use super::basic_definition::{trait_definition::Scalar, definition};

pub fn _lerp
	<V: Sub<Output = V> + Mul<K, Output = V> + Add<Output = V> + Copy, K: Scalar>
	(v1: V, v2: V, p: K) -> V {
	v1 + (v2 - v1) * p
}