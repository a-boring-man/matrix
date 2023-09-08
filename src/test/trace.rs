#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::{definition::Matrix, complex::Complex};

	#[test]
	fn trace() {
		let m1 = Matrix::from((vec![1, 0, 0, 1], 2, 2));
		let m2 = Matrix::from((vec![2, -5, 0, 4, 3, 7, -2, 3, 4], 3, 3));
		let m3 = Matrix::from((vec![-2, -8, 4, 1, -23, 4, 0, 6, 4], 3, 3));
		assert_eq!(2, m1.trace());
		assert_eq!(9, m2.trace());
		assert_eq!(-21, m3.trace());
		let m4 = Matrix::from((vec![Complex{re: 2., im: 1.}, Complex{re: 0., im: 4.2}, Complex{re: -25., im: 0.}, Complex{re: 1., im: -1.}], 2, 2));
		assert_eq!(Complex{re: 3., im: 0.}, m4.trace());
	}
}