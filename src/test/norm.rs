#[cfg(test)]
mod test {
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
}