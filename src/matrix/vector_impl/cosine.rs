use std::{ops::Add, ops::{Mul, Div}, fmt::Display};

use crate::matrix::basic_definition::{trait_definition::{Normable, Complexe}, definition::Vector};

pub fn _angle_cos
	<K: Complexe + Normable + Display + Default + Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K>, const L: usize>
	(v1: Vector<K, L>, v2: Vector<K, L>) -> K {
	v1.dot(v2) / (v1.norm_euclidean() * v2.norm_euclidean())
}

pub fn _angle_cos_complex
	<K: Complexe + Normable + Display + Default + Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K>, const L: usize>
	(v1: Vector<K, L>, v2: Vector<K, L>) -> K {
	v1.complex_dot(v2) / (v1.norm_euclidean() * v2.norm_euclidean())
}