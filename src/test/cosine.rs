#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::{basic_definition::{definition::{Vector, vector}, complex::Complex}, vector_impl::cosine::{_angle_cos, _angle_cos_complex}};

	#[test]
	fn cosine() {
		let v1 = vector([1.0, 0.0]);
		assert_approx_eq!(1.0, _angle_cos(v1, v1) as f64);

		let v2 = vector([0.0, 1.0]);
		assert_approx_eq!(0.0, _angle_cos(v1, v2) as f64);

		let v3 = vector([-1.0, 1.0]);
		let v4 = vector([1.0, -1.0]);
		assert_approx_eq!(-1.0, _angle_cos(v3, v4) as f64);

		let v5 = vector([2.0, 1.0]);
		let v6 = vector([4.0, 2.0]);
		assert_approx_eq!(1.0, _angle_cos(v5, v6) as f64);

		let v7 = vector([1.0, 2.0, 3.0]);
		let v8 = vector([4.0, 5.0, 6.0]);
		assert_approx_eq!(0.974631846, _angle_cos(v7, v8) as f64);

		let v9 = vector([Complex( 1.,  4.), Complex( 3.,  9.)]);
		let v10 = vector([Complex( 2.,  5.), Complex( -2.,  4.)]);
		assert_eq!(Complex( (107.0 as f64).sqrt(),  0.), vector([Complex( 1.,  4.), Complex( 3.,  9.)]).norm_euclidean());
		assert_eq!(Complex( 7.,  0.), vector([Complex( 2.,  5.), Complex( -2.,  4.)]).norm_euclidean());
		assert_eq!(Complex( 52.,  -27.), v9.complex_dot(v10));
		assert_approx_eq!(52. * (107 as f64).sqrt() / 749., _angle_cos_complex(v9, v10).0);
		assert_approx_eq!(-27. * (107 as f64).sqrt() / 749., _angle_cos_complex(v9, v10).1);
	}
} 