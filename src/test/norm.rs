#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{Vector, vector}, complex::Complex};
	
	#[test]
	fn norm_taxicab() {
		let v1: vector<f32, 3> = vector([0.0, 0.0, 0.0]);
		let v2: vector<f64, 3> = vector([1.0, 2.0, 3.0]);
		let v3: vector<f64, 2> = vector([-1.0, -2.0]);
		let v4: vector<Complex, 2> = vector([Complex( 1.,  4.), Complex( 5.,  3.)]);

		assert_eq!(0.0, v1.norm_taxicab());
		assert_eq!(6.0, v2.norm_taxicab());
		assert_eq!(3.0, v3.norm_taxicab());
		assert_approx_eq!(9.95405752, v4.norm_taxicab().0);
	}

	#[test]
	fn norm_euclidean() {
		let v1: vector<f32, 3> = vector([0.0, 0.0, 0.0]);
		let v2: vector<f64, 3> = vector([1.0, 2.0, 3.0]);
		let v3: vector<f64, 2> = vector([-1.0, -2.0]);
		let v4: vector<Complex, 2> = vector([Complex( 1.,  4.), Complex( 5.,  3.)]);

		assert_eq!(0.0, v1.norm_euclidean());
		assert_approx_eq!(3.74165738, v2.norm_euclidean() as f64);
		assert_approx_eq!(2.236067977, v3.norm_euclidean() as f64);
		assert_approx_eq!(7.1414284, v4.norm_euclidean().0);
	}

	#[test]
	fn norm_supremum() {
		let v1 = vector([0.0, 0.0, 0.0]);
		let v2 = vector([1.0, 2.0, 3.0]);
		let v3 = vector([-1.0, -2.0]);
		let v4 = vector([Complex( 1.,  4.), Complex( 5.,  3.)]);

		assert_eq!(0.0, v1.norm_supremum());
		assert_eq!(3.0, v2.norm_supremum());
		assert_eq!(2.0, v3.norm_supremum());
		assert_approx_eq!(5.83095189, v4.norm_supremum().0);
	}
}