use num_traits::Pow;

use super::trait_definition::{Complexe, Normable};

#[derive(Copy, Clone, PartialEq, Debug, Default, PartialOrd)]
pub struct Complex {
	pub(crate) re: f32,
	pub(crate) im: f32,
}

impl Complexe for Complex {
	fn conjugate(&self) -> Self {
		Complex { re: self.re, im: -self.im }
	}
}

impl Normable for Complex {
	fn norm(&self) -> Self {
		Complex { re: (self.re * self.re + self.im * self.im).sqrt(), im: 0. }
	}

	fn square(&self) -> Self {
		*self * *self
	}

	fn square_root(&self) -> Self {
		Complex { re: self.re.sqrt(), im: self.im.sqrt() }
	}
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

impl std::ops::Div for Complex {
	type Output =  Self;

	fn div(self, rhs: Self) -> Self::Output {
		let deno = rhs.norm().re.pow(2);
		Complex {
			re: (self * rhs.conjugate()).re / deno,
			im: (self * rhs.conjugate()).im / deno,
		}
	}
}

impl std::iter::Sum<Complex> for Complex {
	fn sum<I: Iterator<Item = Complex>>(iter: I) -> Self
	where
		I: Iterator<Item = Self>,
	{
		iter.fold(Self::default(), |acc, val| acc + val)
	}
}