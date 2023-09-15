use crate::matrix::basic_definition::{trait_definition::{Scalar, Normable, Complexe}, definition::Vector};

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

