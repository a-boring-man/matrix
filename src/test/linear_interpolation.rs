#[cfg(test)]
mod test {

	use assert_approx_eq::assert_approx_eq;

	use crate::matrix::{basic_definition::{definition::{Vector, Matrix, vector}, complex::Complex}, linear_interpolation::_lerp};

	#[test]
	fn linear_interpolation() {
		let v1 = vector([1., 4.]);
		let v2 = vector([5., 16.]);

		assert_eq!(vector([3., 10.]), _lerp(v1,v2, 0.5));
		assert_eq!(vector([1., 4.]), _lerp(v1,v2, 0.));
		assert_eq!(vector([5., 16.]), _lerp(v1,v2, 1.));

		let c1 = vector([Complex(1., 4.), Complex(-6., 3.)]);
		let c2 = vector([Complex(5., 2.), Complex(2., 5.)]);

		assert_eq!(vector([Complex(3., 3.), Complex(-2., 4.)]), _lerp(c1, c2, Complex(0.5, 0.)));
		assert_eq!(vector([Complex(1., 4.), Complex(-6., 3.)]), _lerp(c1, c2, Complex(0., 0.)));
		assert_eq!(vector([Complex(5., 2.), Complex(2., 5.)]), _lerp(c1, c2, Complex(1., 0.)));
	}
}