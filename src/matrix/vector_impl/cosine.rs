use std::ops::{Add, Mul, Div};

use crate::matrix::basic_definition::{trait_definition::{Normable, Complexe}, definition::Vector};

/// calculate the angle between the two vector
pub fn _angle_cos<K: Normable + Default + Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K>, const L: usize>
	(v1: Vector<K, L>, v2: Vector<K, L>) -> K {
		v1.dot(v2) / (v1.norm_euclidean() * v2.norm_euclidean())
}

/// calculate the complex angle between the two complex vector
pub fn _angle_cos_complex<K: Complexe + Normable + Default + Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K>, const L: usize>
	(v1: Vector<K, L>, v2: Vector<K, L>) -> K {
		v1.complex_dot(v2) / (v1.norm_euclidean() * v2.norm_euclidean())
}