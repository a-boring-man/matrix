#[cfg(test)]
mod test {
    use crate::matrix::basic_definition::definition::Matrix;

	#[test]
	fn projection() {
		let m1 = Matrix::<f32>::projection(90.0, 1., 1.0, 100.0);
		println!("projection : {}", m1);
	}
}