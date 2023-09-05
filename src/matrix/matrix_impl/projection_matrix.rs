use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix};

impl<K: Scalar> Matrix<K> {
	pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
		// |	(h/w)*1/tan(fov/2)				0					0							0 |
		// |					 0	 1/tan(fov/2)					0							0 |
		// |					 0				0	   far/(far-near)    (-far*near)/(far - near) |
		// |					 0				0					1							0 |

		let mut data = vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0.];
		data[0] = 1. / ((fov / 2.).tan() * ratio);
		data[5] = 1. / ((fov / 2.).tan());
		data[10] = far / (far - near);
		data[11] = (-far * near) / (far - near);
		Matrix { data , col: 4, row: 4 }
	}
}