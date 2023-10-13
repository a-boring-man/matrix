#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::matrix, complex::Complex};

	#[test]
	fn reduced_row_echelon() {
		let m1 = matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
		assert_eq!(matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]), m1.row_echelon());

		let m2 = matrix([[1., 2.], [3., 4.]]);
		assert_eq!(matrix([[1., 0.], [0., 1.]]), m2.row_echelon());

		let m3 = matrix([[8., 5., -2., 4., 28.], [4., 2.5, 20., 4., -4.], [8.0, 5.0, 1.0, 4.0, 17.0]]);
		let result = m3.row_echelon();
		let expected = matrix([[1., 0.625, 0., 0., -12.1666667], [0., 0., 1., 0., -3.6666667], [0., 0., 0., 1., 29.5]]);
		for (res, excp) in result.iter().zip(expected.iter()) {
			for (rs, exp) in res.iter().zip(excp.iter()) {
				assert_approx_eq!(*rs as f64, *exp as f64);
			}
		}

		let m4 = matrix([[Complex( 1.,  3.), Complex( 0.,  44.), Complex( -23.,  0.)], [Complex( 50.,  -4.6), Complex( 3.,  42.), Complex( -3.,  -4.2)]]);
		let expected = matrix([[Complex( 1.,  0.), Complex( 0.,  0.), Complex( 0.39687,  -0.05800)], [Complex( 0.,  0.), Complex( 1.,  0.), Complex( -0.02574,  0.53570)]]);
		let result = m4.row_echelon();
		//println!("result {}", result);
		for (res,excp) in result.iter().zip(expected.iter()) {
			for (rs, exp) in res.iter().zip(excp.iter()) {
				assert_approx_eq!(rs.0, exp.0, 1e-2);
				assert_approx_eq!(rs.1, exp.1, 1e-2);
			}
		}

		let m5 = matrix([[8.0, 5.0, -2., 4., 28.], [4., 2.5, 20., 4., -4.], [8., 5., 1., 4., 17.]]);
		let expected = matrix([[1., 0.625, 0., 0., -12.166666666666666], [0., 0., 1., 0., -3.6666666666666666666], [0., 0., 0., 1., 29.5]]);
		let result = m5.row_echelon();
		for r in 0..3 {
			for c in 0..5 {
				assert_approx_eq!(result.0[r][c] as f64, expected.0[r][c] as f64);
			}
		}
	}
}