#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{Matrix, Vector}, complex::Complex};
	#[test]
	fn matrix_matrix_multiplication() {
		let m1 = Matrix::from((vec![3303.0, 45345.0, 34.0, 4.542, 0.254, 453.0, 37.0, 453.0, 65.0, 6.0, 766.0, 54.0, 452.0, 354.0, 65.0],3 ,5 ));
		let m2 = Matrix::from((vec![45.0, 65.0, 86.0, 737.0, 345.0, 32.0, 365.0, 86.0, 67.0, 453.0, 35.0, 12.0, 4.0, 3534.0, 654.0, 45.0, 6.0, 34.0], 6, 3));
		let expect_result = Matrix::from((vec![16699696.0, 4234521.0, 3344409.0, 22977126.0, 2726814.0, 650992.0, 2109.1, 1601219.074, 296669.63, 23847.516, 4293.88, 15550.392, 167270.0, 271073.0, 76043.0, 235403.0, 29010.0, 8830.0, 280076.0, 257102.0, 87154.0, 353850.0, 29204.0, 11220.0, 149810.0, 289534.0, 105100.0, 496411.0, 168720.0, 20922.0], 6, 5));
		let result = m1.mul_mat(&m2);
		for i in 0..result.len() {
			assert_approx_eq!(expect_result.data[i] as f64, result.data[i] as f64);
		}
		let m3 = Matrix::from((vec![Complex{re: 1.,  7.}, Complex{re: 55.,  5.}, Complex{re: 54.,  24.}, Complex{re: 5.1,  51.}, Complex{re: 56.,  0.}, Complex{re: 98.,  15.}, Complex{re: 58.12,  578.}, Complex{re: 42.,  5.}, Complex{re: 6.,  -3.}], 3, 3));
		let m4 = Matrix::from((vec![Complex{re: 947.,  1.}, Complex{re: 35.5,  3.6}, Complex{re: 97.,  -25.}, Complex{re: 0.,  89.}, Complex{re: 0.,  47.}, Complex{re: 0.,  42.42}, Complex{re: 233.2,  0.}, Complex{re: 32.,  43.}, Complex{re: 0.,  32.}], 3, 3));
		let expected_result = Matrix::from((vec![Complex{re: 13087.8,  17121.8}, Complex{re: 471.3,  5927.1}, Complex{re: -708.1,  4715.1}, Complex{re: 27632.3,  56784.1}, Complex{re: 2488.45,  9154.86}, Complex{re: 1289.7,  10331.02}, Complex{re: 55415.84,  550462.52}, Complex{re: 68.46,  22864.232}, Complex{re: 19971.54,  56586.64}], 3, 3));
		let result = m3.mul_mat(&m4);
		for i in 0..result.len() {
			assert_approx_eq!(expected_result.data[i].re as f64, result.data[i].re as f64, 1e-2);
			assert_approx_eq!(expected_result.data[i].im as f64, result.data[i].im as f64, 1e-2);
		}
	}

	#[test]
	fn matrix_vector_multiplication() {
		let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
		let v1 = Vector::from(vec![4, 2]);
		let v2 = Vector::from(vec![Complex{re: 1.,  3.}, Complex{re: 4.,  -3.5}]);
		let m2 = Matrix::from((vec![Complex{re: 2.,  0.}, Complex{re: 0.,  0.}, Complex{re: 0.,  0.}, Complex{re: 2.,  0.}], 2, 2));
		assert_eq!(Vector::from(vec![8, 20]), m1.mul_vec(&v1));
		assert_eq!(Vector::from(vec![Complex{re: 2.,  6.}, Complex{re: 8.,  -7.}]), m2.mul_vec(&v2));
	}
}