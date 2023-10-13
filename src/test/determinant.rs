#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::{definition::{matrix}, complex::Complex};

	#[test]
	fn determinant() {
		let m1 = matrix([[1, 2], [3, 4]]);
		assert_eq!(-2, m1.determinant());

		let m2 = matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
		assert_eq!(0, m2.determinant());

		let m3 = matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
		assert_eq!(0, m3.determinant());

		let m4 = matrix([[Complex( 0.,  3.), Complex( 4.,  2.)], [Complex( -5.,  0.), Complex( 3.,  1.)]]);
		assert_eq!(Complex( 17.,  19.), m4.determinant());

		let m5 = matrix([[1, 2], [3, 4]]);
		assert_eq!(-2, m5.determinant());
	}
}