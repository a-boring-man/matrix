#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::{definition::{Matrix, matrix}, complex::Complex};

	#[test]
	fn rank() {
		let m1 = matrix([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
		assert_eq!(m1.rank(), 3);

		let m2 = matrix([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
		assert_eq!(m2.rank(), 2);

		let m3 = matrix([[8., 5., -2., 4.], [7., 20., 7., 6.], [1., 21., 18., 7.]]);
		assert_eq!(m3.rank(), 3);

		let m4 = matrix([[Complex(3., 2.), Complex(1.5, 1.)], [Complex(2., 4.), Complex(1., 2.)]]);
		assert_eq!(m4.rank(), 1);
	}
}