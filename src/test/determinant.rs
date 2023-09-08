#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::{definition::Matrix, complex::Complex};

	#[test]
	fn determinant() {
		let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
		assert_eq!(-2, m1.determinant().unwrap());

		let m2 = Matrix::from((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3));
		assert_eq!(0, m2.determinant().unwrap());

		let m3 = Matrix::from((vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 4, 4));
		assert_eq!(0, m3.determinant().unwrap());

		let m4 = Matrix::from((vec![Complex{re: 0., im: 3.}, Complex{re: 4., im: 2.}, Complex{re: -5., im: 0.}, Complex{re: 3., im: 1.}], 2, 2));
		assert_eq!(Complex{re: 17., im: 19.}, m4.determinant().unwrap());
	}
}