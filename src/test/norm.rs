#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{Vector}, complex::Complex};
	
	#[test]
	fn norm_taxicab() {
		let v1: Vector<f32, 3> = Vector([0.0, 0.0, 0.0]);
		let v2: Vector<f64, 3> = Vector([1.0, 2.0, 3.0]);
		let v3: Vector<f64, 2> = Vector([-1.0, -2.0]);
		let v4: Vector<Complex, 2> = Vector([Complex( 1.,  4.), Complex( 5.,  3.)]);

		assert_eq!(0.0, v1.norm_taxicab());
		assert_eq!(6.0, v2.norm_taxicab());
		assert_eq!(3.0, v3.norm_taxicab());
		assert_approx_eq!(9.95405752, v4.norm_taxicab().0);
	}

	#[test]
	fn norm_euclidean() {
		let v1: Vector<f32, 3> = Vector([0.0, 0.0, 0.0]);
		let v2: Vector<f64, 3> = Vector([1.0, 2.0, 3.0]);
		let v3: Vector<f64, 2> = Vector([-1.0, -2.0]);
		let v4: Vector<Complex, 2> = Vector([Complex( 1.,  4.), Complex( 5.,  3.)]);

		assert_eq!(0.0, v1.norm_euclidean());
		assert_approx_eq!(3.74165738, v2.norm_euclidean() as f64);
		assert_approx_eq!(2.236067977, v3.norm_euclidean() as f64);
		assert_approx_eq!(7.1414284, v4.norm_euclidean().0);
	}

	#[test]
	fn norm_supremum() {
		let v1 = Vector([0.0, 0.0, 0.0]);
		let v2 = Vector([1.0, 2.0, 3.0]);
		let v3 = Vector([-1.0, -2.0]);
		let v4 = Vector([Complex( 1.,  4.), Complex( 5.,  3.)]);

		assert_eq!(0.0, v1.norm_supremum());
		assert_eq!(3.0, v2.norm_supremum());
		assert_eq!(2.0, v3.norm_supremum());
		assert_approx_eq!(5.83095189, v4.norm_supremum().0);
	}
}