use super::definition::{Matrix, Scalar, Vector};

impl<K: Scalar> Matrix<K> {
	pub fn lerp(m1: &Matrix<K>, m2: &Matrix<K>, r: &K) ->Matrix<K> {
		m1.add(&m2.sub(m1).scale(r))
	}
}

impl<K: Scalar> Vector<K> {
	pub fn lerp(v1: &Vector<K>, v2: &Vector<K>, r: &K) ->Vector<K> {
		v1.add(&v2.sub(v1).scale(r))
	}
}