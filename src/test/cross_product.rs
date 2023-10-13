#[cfg(test)]
mod test {
    use crate::matrix::{basic_definition::{definition::{Vector}, complex::Complex}, vector_impl::cross_product::_cross_product};

	#[test]
	fn cross_product() {
		let v1: Vector<i32, 3> = Vector([0, 0, 1]);
		let v2: Vector<i32, 3> = Vector([1, 0, 0]);
		assert_eq!(Vector([0, 1, 0]), _cross_product(v1, v2));

		let u: Vector<f64, 3> = Vector([1.0, 2.0, 3.0]);
		let v: Vector<f64, 3> = Vector([4.0, 5.0, 6.0]);
		assert_eq!(Vector([-3.0, 6.0, -3.0]), _cross_product(u, v));

		let u1: Vector<f32, 3> = Vector([4.0, 2.0, -3.0]);
		let v1: Vector<f32, 3> = Vector([-2.0, -5.0, 16.0]);
		assert_eq!(Vector([17.0, -58.0, -16.0]), _cross_product(u1, v1));

		let v9 = Vector([Complex( 1.,  4.), Complex( 3.,  9.), Complex( 5.,  3.)]);
		let v10 = Vector([Complex( 2.,  5.), Complex( -2.,  4.), Complex( 4.,  7.)]);
		assert_eq!(Vector([Complex( -29.,  43.), Complex( 19.,  8.), Complex( 21.,  -37.)]), _cross_product(v9, v10));
	}
}