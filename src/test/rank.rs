#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::definition::{Matrix, matrix};

	#[test]
	fn rank() {
		let m1 = Matrix::from((vec![1, 0, 0, 0, 1, 0, 0, 0, 1], 3, 3));
		assert_eq!(m1.rank(), 3);

		let m2 = Matrix::from((vec![1., 2., 0., 0., 2., 4., 0., 0., -1., 2., 1., 1.], 4, 3));
		assert_eq!(m2.rank(), 2);

		let m3 = Matrix::from((vec![8., 5., -2., 4., 7., 20., 7., 6., 1., 21., 18., 7.], 3, 4));
		assert_eq!(m3.rank(), 3);

		let m1 = matrix([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
		assert_eq!(m1.rank(), 3);

		let m2 = matrix([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
		assert_eq!(m2.rank(), 2);

		let m3 = matrix([[8., 5., -2., 4.], [7., 20., 7., 6.], [1., 21., 18., 7.]]);
		assert_eq!(m3.rank(), 3);
	}
}