#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{vector}, complex::Complex};

	#[test]
	fn dot() {
		let v1 = vector([0., 0.]);
		let v2 = vector([1., 1.]);
		let v3 = vector([-1., 6.]);
		let v4 = vector([3., 2.]);
		let v5 = vector([-1, 6]);
		let v6 = vector([3, 2]);
		let c1 = vector([Complex(1., 4.), Complex(-6., 3.)]);
		let c2 = vector([Complex(5., 2.), Complex(2., 5.)]);

		assert_approx_eq!(0. as f64, v1.dot(v2));
		assert_approx_eq!(2. as f64, v2.dot(v2));
		assert_approx_eq!(9. as f64, v3.dot(v4));
		assert_eq!(9, v5.dot(v6));
		assert_eq!(Complex(16., 54.), c1.complex_dot(c2));
	}
}