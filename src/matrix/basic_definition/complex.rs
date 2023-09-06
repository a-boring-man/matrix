#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Complex {
	pub(crate) re: f32,
	pub(crate) im: f32,
}

impl std::ops::Add for Complex {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Complex {
			re: self.re + rhs.re,
			im: self.im + rhs.im
		}
	}
}

impl std::ops::Sub for Complex {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Complex{
			re: self.re - rhs.re,
			im: self.im - rhs.im,
		}
	}
}

impl std::ops::Mul for Complex {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Complex {
			re: self.re * rhs.re - self.im * rhs.im,
			im: self.re * rhs.im + self.im * rhs.re,
		}
	}
}