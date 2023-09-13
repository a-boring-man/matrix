#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::Vector, complex::Complex};

	#[test]
	fn cosine() {
		let v1 = Vector::from(vec![1.0, 0.0]);
		assert_approx_eq!(1.0, Vector::angle_cos(&v1, &v1) as f64);

		let v2 = Vector::from(vec![0.0, 1.0]);
		assert_approx_eq!(0.0, Vector::angle_cos(&v1, &v2) as f64);

		let v3 = Vector::from(vec![-1.0, 1.0]);
		let v4 = Vector::from(vec![1.0, -1.0]);
		assert_approx_eq!(-1.0, Vector::angle_cos(&v3, &v4) as f64);

		let v5 = Vector::from(vec![2.0, 1.0]);
		let v6 = Vector::from(vec![4.0, 2.0]);
		assert_approx_eq!(1.0, Vector::angle_cos(&v5, &v6) as f64);

		let v7 = Vector::from(vec![1.0, 2.0, 3.0]);
		let v8 = Vector::from(vec![4.0, 5.0, 6.0]);
		assert_approx_eq!(0.974631846, Vector::angle_cos(&v7, &v8) as f64);

		let v9 = Vector::from(vec![Complex( 1.,  4.), Complex( 3.,  9.)]);
		let v10 = Vector::from(vec![Complex( 2.,  5.), Complex( -2.,  4.)]);
		assert_eq!(Complex( (107.0 as f32).sqrt(),  0.), Vector::from(vec![Complex( 1.,  4.), Complex( 3.,  9.)]).norm_euclidean());
		assert_eq!(Complex( 7.,  0.), Vector::from(vec![Complex( 2.,  5.), Complex( -2.,  4.)]).norm_euclidean());
		assert_eq!(Complex( 52.,  -27.), v9.dot_complex(&v10));
		assert_approx_eq!(52. * (107 as f32).sqrt() / 749., Vector::angle_cos_complex(&v9, &v10).0);
		assert_approx_eq!(-27. * (107 as f32).sqrt() / 749., Vector::angle_cos_complex(&v9, &v10).1);
	}
} 