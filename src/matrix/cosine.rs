use super::{definition::{Vector}, trait_definition::{Scalar, Normable}};

impl<K: Scalar + Normable + std::ops::Div<Output = K>> Vector<K> {
	pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K {
		u.dot(v) / (u.norm_euclidean() * v.norm_euclidean())
	}
}