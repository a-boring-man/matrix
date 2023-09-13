#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::{definition::Vector, complex::Complex};

	#[test]
	fn dot() {
		let v1 = Vector::from(vec![0.0, 0.0]);
		let v2 = Vector::from(vec![1.0, 1.0]);
		let v3 = Vector::from(vec![-1.0, 6.0]);
		let v4 = Vector::from(vec![3.0, 2.0]);
		let v5 = Vector::from(vec![-1, 6]);
		let v6 = Vector::from(vec![3, 2]);
		let v7 = Vector::from(vec![Complex{re: 1.0,  4.0}, Complex{re: -6.,  3.}]);
		let v8 = Vector::from(vec![Complex{re: 5.,  2.}, Complex{re: 2.0,  5.0}]);

		assert_eq!(0.0, v1.dot(&v2));
		assert_eq!(2.0, v2.dot(&v2));
		assert_eq!(9.0, v3.dot(&v4));
		assert_eq!(9, v5.dot(&v6));
		assert_eq!(Complex{re: 16.,  54.}, v7.dot_complex(&v8));
	}
}