#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::definition::Matrix;

	#[test]
	fn determinant() {
		let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
		assert_eq!(-5, m1.determinant_2d());

		let m2 = Matrix::from((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3));
		assert_eq!(0, m2.determinant_3d());
	}
}