// use matrix::matrix::basic_definition::definition::Matrix;

use matrix::matrix::basic_definition::definition::matrix;

fn main() {
	// let m = Matrix::from((vec![1, 2, 3, 4], 2, 2));
	// let m2 = Matrix {data: vec![1, 2, 3, 4], col: 2, row: 2};
	// println!("test {}", m.da);
	let m1 = matrix([[0; 0]]);
	let m2 = matrix([[0; 0]; 0]);
	println!("coucou");
	for i in 0..10 {
		println!("{}", m2.0[i][i]);
	}
	println!("salut");
}