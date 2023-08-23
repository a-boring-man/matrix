#[cfg(test)]
mod test {
    use crate::matrix::definition::Matrix;

	#[test]
	fn determinant_2d() {
		let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
		assert_eq!(-5, m1.determinant_2d());
	}
}