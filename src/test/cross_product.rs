#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::{definition::Vector, complex::Complex};

	#[test]
	fn cross_product() {
		let v1 = Vector::from(vec![0.0, 0.0, 1.0]);
		let v2 = Vector::from(vec![1.0, 0.0, 0.0]);
		assert_eq!(Vector::from(vec![0.0, 1.0, 0.0]), Vector::cross_product(&v1, &v2));

		let u = Vector::from(vec![1.0, 2.0, 3.0]);
		let v = Vector::from(vec![4.0, 5.0, 6.0]);
		assert_eq!(Vector::from(vec![-3.0, 6.0, -3.0]), Vector::cross_product(&u, &v));

		let u1 = Vector::from(vec![4.0, 2.0, -3.0]);
		let v1 = Vector::from(vec![-2.0, -5.0, 16.0]);
		assert_eq!(Vector::from(vec![17.0, -58.0, -16.0]), Vector::cross_product(&u1, &v1));

		let v9 = Vector::from(vec![Complex( 1.,  4.), Complex( 3.,  9.), Complex( 5.,  3.)]);
		let v10 = Vector::from(vec![Complex( 2.,  5.), Complex( -2.,  4.), Complex( 4.,  7.)]);
		assert_eq!(Vector::from(vec![Complex( -29.,  43.), Complex( 19.,  8.), Complex( 21.,  -37.)]), Vector::cross_product(&v9, &v10));
	}
}