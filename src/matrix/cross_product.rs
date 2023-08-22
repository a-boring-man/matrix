use super::definition::{Scalar, Vector};
use std::ops::Mul;

impl<K: Scalar> Vector<K> where for<'a> &'a K: Mul<&'a K, Output = K>{
	pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
		if u.v.len() != 3 || v.v.len() != 3 {
			panic!("cross product call on a non 3d vector");
		}
		Vector::from(vec![&u.v[1] * &v.v[2] - &u.v[2] * &v.v[1], &u.v[2] * &v.v[0] - &u.v[0] * &v.v[2], &u.v[0] * &v.v[1] - &u.v[1] * &v.v[0]])
	}
}