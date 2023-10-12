#[cfg(test)]
mod test {
    use crate::matrix::{basic_definition::{definition::{Vector, vector}, complex::Complex}, vector_impl::cross_product::_cross_product};

	#[test]
	fn cross_product() {
		let v1: vector<i32, 3> = vector([0, 0, 1]);
		let v2: vector<i32, 3> = vector([1, 0, 0]);
		assert_eq!(vector([0, 1, 0]), _cross_product(v1, v2));

		let u: vector<f64, 3> = vector([1.0, 2.0, 3.0]);
		let v: vector<f64, 3> = vector([4.0, 5.0, 6.0]);
		assert_eq!(vector([-3.0, 6.0, -3.0]), _cross_product(u, v));

		let u1: vector<f32, 3> = vector([4.0, 2.0, -3.0]);
		let v1: vector<f32, 3> = vector([-2.0, -5.0, 16.0]);
		assert_eq!(vector([17.0, -58.0, -16.0]), _cross_product(u1, v1));

		let v9 = vector([Complex( 1.,  4.), Complex( 3.,  9.), Complex( 5.,  3.)]);
		let v10 = vector([Complex( 2.,  5.), Complex( -2.,  4.), Complex( 4.,  7.)]);
		assert_eq!(vector([Complex( -29.,  43.), Complex( 19.,  8.), Complex( 21.,  -37.)]), _cross_product(v9, v10));
	}
}