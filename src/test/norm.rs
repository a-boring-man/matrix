#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::definition::Vector;

	#[test]
	fn norm_taxicab() {
		let v1 = Vector::from(vec![0.0, 0.0, 0.0]);
		let v2 = Vector::from(vec![1.0, 2.0, 3.0]);
		let v3 = Vector::from(vec![-1.0, -2.0]);

		assert_eq!(0.0, v1.norm_taxicab());
		assert_eq!(6.0, v2.norm_taxicab());
		assert_eq!(3.0, v3.norm_taxicab());
	}

	#[test]
	fn norm_euclidean() {
		let v1 = Vector::from(vec![0.0, 0.0, 0.0]);
		let v2 = Vector::from(vec![1.0, 2.0, 3.0]);
		let v3 = Vector::from(vec![-1.0, -2.0]);

		assert_eq!(0.0, v1.norm_euclidean());
		assert_approx_eq!(3.74165738, v2.norm_euclidean() as f64);
		assert_approx_eq!(2.236067977, v3.norm_euclidean() as f64);
	}

	#[test]
	fn norm_supremum() {
		let v1 = Vector::from(vec![0.0, 0.0, 0.0]);
		let v2 = Vector::from(vec![1.0, 2.0, 3.0]);
		let v3 = Vector::from(vec![-1.0, -2.0]);

		assert_eq!(0.0, v1.norm_supremum());
		assert_eq!(3.0, v2.norm_supremum());
		assert_eq!(2.0, v3.norm_supremum());
	}
}