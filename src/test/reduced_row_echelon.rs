#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::definition::Matrix;

	#[test]
	fn reduced_row_echelon() {
		let m1 = Matrix::from((vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], 3, 3));
		assert_eq!(Matrix::from((vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], 3, 3)), m1.row_echelon());

		let m2 = Matrix::from((vec![1.0, 2.0, 3.0, 4.0,], 2, 2));
		assert_eq!(Matrix::from((vec![1.0, 0.0, 0.0, 1.0], 2, 2)), m2.row_echelon());

		let m3 = Matrix::from((vec![8.0, 5.0, -2.0, 4.0, 28.0, 4.0, 2.5, 20.0, 4.0, -4.0, 8.0, 5.0, 1.0, 4.0, 17.0], 5, 3));
		let result = m3.row_echelon();
		let expected = Matrix::from((vec![1.0, 0.625, 0.0, 0.0, -12.1666667, 0.0, 0.0, 1.0, 0.0, -3.6666667, 0.0, 0.0, 0.0, 1.0, 29.5], 5, 3));
		for i in 0..result.data.len() {
			assert_approx_eq!(result.data[i as usize] as f64, expected.data[i as usize]);
		}
	}
}