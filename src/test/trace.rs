#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::{definition::{matrix}, complex::Complex};

	#[test]
	fn trace() {
		let m1 = matrix([[1, 0], [0, 1]]);
		let m2 = matrix([[2, -5, 0], [4, 3, 7], [-2, 3, 4]]);
		let m3 = matrix([[-2, -8, 4], [1, -23, 4], [0, 6, 4]]);
		assert_eq!(2, m1.trace());
		assert_eq!(9, m2.trace());
		assert_eq!(-21, m3.trace());
		let m4 = matrix([[Complex( 2.,  1.), Complex( 0.,  4.2)], [Complex( -25.,  0.), Complex( 1.,  -1.)]]);
		assert_eq!(Complex( 3.,  0.), m4.trace());
	}
}