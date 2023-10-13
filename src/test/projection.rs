#[cfg(test)]
mod test {
    use crate::matrix::matrix_impl::projection_matrix::_projection;


	#[test]
	fn projection() {
		let m1 = _projection(90.0, 1., 1.0, 100.0);
		println!("projection : {}", m1);
	}
}