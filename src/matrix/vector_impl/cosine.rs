use crate::matrix::basic_definition::{trait_definition::{Scalar, Normable}, definition::Vector};

impl<K: Scalar + Normable + std::ops::Div<Output = K>> Vector<K> {
	pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K {
		u.dot(v) / (u.norm_euclidean() * v.norm_euclidean())
	}
}