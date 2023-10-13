use crate::matrix::basic_definition::definition::matrix;

pub fn _projection
	<K>
	(fov: f64, ratio: f64, near: f64, far: f64) -> matrix<f64, 4, 4> {

		// |	(h/w)*1/tan(fov/2)				0					0							0 |
		// |					 0	 1/tan(fov/2)					0							0 |
		// |					 0				0	  -far/(far-near)    -(far*near)/(far - near) |
		// |					 0				0				   -1							0 |

		let mut m = matrix([[0.; 4]; 4]);
		let fovr = fov.to_radians();
		m.0[3][2] = -1.;
		m.0[0][0] = 1. / ((fovr / 2.).tan() * ratio);
		m.0[1][1] = 1. / ((fovr / 2.).tan());
		m.0[2][2] = far / (near - far);
		m.0[2][3] = -(far * near) / (far - near);
		m
	}