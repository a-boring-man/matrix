#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::definition::{Matrix, Vector};
	#[test]
	fn matrix_matrix_multiplication() {
		let m1 = Matrix::from((vec![3303.0, 45345.0, 34.0, 4.542, 0.254, 453.0, 37.0, 453.0, 65.0, 6.0, 766.0, 54.0, 452.0, 354.0, 65.0],3 ,5 ));
		let m2 = Matrix::from((vec![45.0, 65.0, 86.0, 737.0, 345.0, 32.0, 365.0, 86.0, 67.0, 453.0, 35.0, 12.0, 4.0, 3534.0, 654.0, 45.0, 6.0, 34.0], 6, 3));
		let expect_result = Matrix::from((vec![16699696.0, 4234521.0, 3344409.0, 22977126.0, 2726814.0, 650992.0, 2109.1, 1601219.074, 296669.63, 23847.516, 4293.88, 15550.392, 167270.0, 271073.0, 76043.0, 235403.0, 29010.0, 8830.0, 280076.0, 257102.0, 87154.0, 353850.0, 29204.0, 11220.0, 149810.0, 289534.0, 105100.0, 496411.0, 168720.0, 20922.0], 6, 5));
		let result = m1.mul_mat(&m2);
		for i in 0..result.len() {
			assert_approx_eq!(expect_result.data[i] as f64, result.data[i] as f64);
		}
	}

	#[test]
	fn matrix_vector_multiplication() {
		let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
		let v1 = Vector::from(vec![4, 2]);
		assert_eq!(Vector::from(vec![8, 20]), m1.mul_vec(&v1));
	}
}