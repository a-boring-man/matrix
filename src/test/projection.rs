#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::definition::Matrix;

	#[test]
	fn projection() {
		let m1 = Matrix::<f32>::projection(1.7, 1., 0.00001, 100.);
		println!("projection : {}", m1);
	}
}