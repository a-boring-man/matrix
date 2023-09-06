pub mod add_sub_scale;
pub mod linear_interpolation;

pub mod basic_definition {
	pub mod definition;
	pub mod error;
	pub mod display;
	pub mod trait_definition;
	pub mod utils;
	pub mod iterator_impl;
	pub mod complex;
}

pub mod matrix_impl {
	pub mod determinant;
	pub mod matrix_multiplication;
	pub mod reduced_row_echelon;
	pub mod trace;
	pub mod transpose;
	pub mod inverse;
	pub mod rank;
	pub mod projection_matrix;
}

pub mod vector_impl {
	pub mod cosine;
	pub mod cross_product;
	pub mod dot;
	pub mod linear_combination;
	pub mod norm;
}