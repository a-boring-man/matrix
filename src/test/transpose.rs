#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::definition::{Matrix};

	#[test]
	fn transpose() {
		let m1 = Matrix([[5, 6], [8, 7]]);
		assert_eq!(Matrix([[5, 8], [6, 7]]), m1.transpose());
		let m2 = Matrix([[5, 6], [8, 7], [42, 1]]);
		assert_eq!(Matrix([[5, 8, 42], [6, 7, 1]]), m2.transpose());
		let m3 = Matrix([[56, 156, 13, 8, 4], [6, 1, 3, 8, 4], [2, 3, 8, 1, 36], [8, 1, 2, 3, 5], [69, 5, 1, 9, 8], [7, 3, 3, 1, 5]]);
		assert_eq!(Matrix([[56,6,2,8,69,7],[156,1,3,1,5,3],[13,3,8,2,1,3],[8,8,1,3,9,1],[4,4,36,5,8,5]]), m3.transpose());
		let m4 = Matrix([[5, 6, 8, 7]]);
		assert_eq!(Matrix([[5], [6], [8], [7]]), m4.transpose());
	}
}