#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{Matrix, Vector}, complex::Complex};
	#[test]
	fn matrix_matrix_multiplication() {
		let m1: Matrix<f64, 5, 3> = Matrix([[3303. , 45345. , 34. ], [4.542, 0.254, 453. ], [37. , 453. , 65. ], [6. , 766. , 54. ], [452. , 354. , 65. ]]);
		let m2: Matrix<f64, 3, 6> = Matrix([[45. , 65. , 86. , 737. , 345. , 32.] , [365. , 86. , 67. , 453. , 35. , 12.] , [4. , 3534. , 654. , 45. , 6. , 34. ]]);
		let expect_result: Matrix<f64, 5, 6> = Matrix([[16699696. , 4234521. , 3344409. , 22977126. , 2726814. , 650992.] , [2109.1, 1601219.074, 296669.63, 23847.516, 4293.88, 15550.392], [167270. , 271073. , 76043. , 235403. , 29010. , 8830.] , [280076. , 257102. , 87154. , 353850. , 29204. , 11220.] , [149810. , 289534. , 105100. , 496411. , 168720. , 20922.]]);
		let result: Matrix<f64, 5, 6> = m1 * m2;
		for (res, excpt) in result.iter().zip(expect_result.iter()) {
			for (rs, exp) in res.iter().zip(excpt.iter()) {
				assert_approx_eq!(*rs as f64, *exp as f64);
			}
		}

		let m3 = Matrix([[Complex( 1., 7.), Complex( 55., 5.), Complex( 54.,  24.)], [Complex( 5.1,  51.), Complex( 56.,  0.), Complex( 98.,  15.)], [Complex( 58.12,  578.), Complex( 42.,  5.), Complex( 6.,  -3.)]]);
		let m4 = Matrix([[Complex( 947., 1.), Complex( 35.5, 3.6), Complex( 97.,  -25.)], [Complex( 0.,  89.), Complex( 0.,  47.), Complex( 0.,  42.42)], [Complex( 233.2,  0.), Complex( 32.,  43.), Complex( 0.,  32.)]]);
		let expected_result = Matrix([[Complex( 13087.8,  17121.8), Complex( 471.3,  5927.1), Complex( -708.1,  4715.1)], [Complex( 27632.3,  56784.1), Complex( 2488.45,  9154.86), Complex( 1289.7,  10331.02)], [Complex( 55415.84,  550462.52), Complex( 68.46,  22864.232), Complex( 19971.54,  56586.64)]]);
		let result = m3 * m4;
		for (res, excp) in result.iter().zip(expected_result.iter()) {
			for (rs, exp) in res.iter().zip(excp.iter()) {
				assert_approx_eq!(rs.0 as f64, exp.0 as f64, 1e-2);
				assert_approx_eq!(rs.1 as f64, exp.1 as f64, 1e-2);
			}
		}
	}

	#[test]
	fn matrix_vector_multiplication() {
		let m1 = Matrix([[1, 2], [3, 4]]);
		let v1 = Vector([4, 2]);
		let v2 = Vector([Complex( 1.,  3.), Complex( 4.,  -3.5)]);
		let m2 = Matrix([[Complex( 2.,  0.), Complex( 0.,  0.)], [Complex( 0.,  0.), Complex( 2.,  0.)]]);
		assert_eq!(Vector([8, 20]), m1 * v1);
		assert_eq!(Vector([Complex( 2.,  6.), Complex( 8.,  -7.)]), m2 * v2);
	}
}