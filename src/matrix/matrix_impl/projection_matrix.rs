use crate::matrix::basic_definition::{trait_definition::Scalar, definition::Matrix};

impl<K: Scalar> Matrix<K> {
	pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
		// |	(h/w)*1/tan(fov/2)				0					0							0 |
		// |					 0	 1/tan(fov/2)					0							0 |
		// |					 0				0	   far/(far-near)    (far*near)/(far - near) |
		// |					 0				0					1							0 |

		let mut data = vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0.];
		println!("{:?}", data);
		data[0] = 1. / ((fov / 2.).tan() * ratio);
		println!("{:?}", data);
		data[5] = 1. / ((fov / 2.).tan());
		println!("{:?}", data);
		data[10] = far / (far - near);
		println!("{:?}", data);
		data[11] = (far * near) / (far - near);
		println!("{:?}", data);
		println!("{}", Matrix { data: data.clone() , col: 4, row: 4 });
		Matrix { data , col: 4, row: 4 }
	}
}