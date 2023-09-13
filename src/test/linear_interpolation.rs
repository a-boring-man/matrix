#[cfg(test)]
mod test {

	use assert_approx_eq::assert_approx_eq;

	use crate::matrix::basic_definition::{definition::{Vector, Matrix}, complex::Complex};

	#[test]
	fn linear_interpolation() {
		let v1 = Vector::from(vec![1.0, 4.0]);
		let v2 = Vector::from(vec![5.0, 16.0]);
		let v3 = Vector::from(vec![Complex{re: 1.0,  4.0}, Complex{re: -6.,  3.}]);
		let v4 = Vector::from(vec![Complex{re: 5.,  2.}, Complex{re: 2.0,  5.0}]);

		let m1 = Matrix::from((vec![10.0, -20.0, 4.0, -3.0], 2, 2));
		let m2 = Matrix::from((vec![25.0, 42.0, 64.0, -7.0], 2, 2));

		assert_eq!(Vector::from(vec![3.0, 10.0]), Vector::lerp(&v1, &v2, &0.5));
		assert_eq!(Vector::from(vec![1.0, 4.0]), Vector::lerp(&v1, &v2, &0.0));
		assert_eq!(Vector::from(vec![5.0, 16.0]), Vector::lerp(&v1, &v2, &1.0));

		assert_eq!(Vector::from(vec![Complex{re: 3.,  3.0}, Complex{re: -2.,  4.}]), Vector::lerp(&v3, &v4, &Complex{re: 0.5,  0.0}));
		assert_eq!(Vector::from(vec![Complex{re: 1.0,  4.0}, Complex{re: -6.,  3.}]), Vector::lerp(&v3, &v4, &Complex{re: 0.0,  0.0}));
		assert_eq!(Vector::from(vec![Complex{re: 5.,  2.}, Complex{re: 2.0,  5.0}]), Vector::lerp(&v3, &v4, &Complex{re: 1.0,  0.0}));

		assert_eq!(Matrix::from((vec![10.0, -20.0, 4.0, -3.0], 2, 2)), Matrix::lerp(&m1, &m2, 0.0));
		assert_eq!(Matrix::from((vec![25.0, 42.0, 64.0, -7.0], 2, 2)), Matrix::lerp(&m1, &m2, 1.0));
		let lerp = Matrix::lerp(&m1, &m2, 0.3);
		let expected_result = Matrix::from((vec![14.5, -1.4, 22.0, -4.2], 2, 2));
		let dit = lerp.iter().zip(expected_result.iter());
		for (val1, val2) in dit {
			assert_approx_eq!(*val1 as f64, *val2 as f64);
		}
	}
}