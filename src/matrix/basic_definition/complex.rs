use num_traits::Pow;

use num_traits::One;

use super::trait_definition::CanDoaDotProduct;
use super::trait_definition::{Complexe, Normable};

#[derive(Copy, Clone, PartialEq, Debug, Default, PartialOrd)]
pub struct Complex (pub f32,pub f32);

impl Complexe for Complex {
	fn conjugate(&self) -> Self {
		Complex(self.0, -self.1)
	}
}

impl Normable for Complex {
	fn norm(&self) -> Self {
		Complex((self.0 * self.0 + self.1 * self.1).sqrt(), 0.)
	}

	fn square(&self) -> Self {
		*self * *self
	}

	fn square_root(&self) -> Self {
		Complex( self.0.sqrt(), self.1.sqrt())
	}
}

impl std::ops::Add for Complex {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Complex( self.0 + rhs.0,
			 self.1 + rhs.1 )
	}
}
impl std::ops::Sub for Complex {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Complex(self.0 - rhs.0, self.1 - rhs.1,)
	}
}
impl std::ops::Mul for Complex {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Complex( self.0 * rhs.0 - self.1 * rhs.1,
			 self.0 * rhs.1 + self.1 * rhs.0)
	}
}
impl std::ops::Div for Complex {
	type Output =  Self;

	fn div(self, rhs: Self) -> Self::Output {
		let deno = rhs.norm().0.pow(2);
		Complex( (self * rhs.conjugate()).0 / deno,
			 (self * rhs.conjugate()).1 / deno)
	}
}

impl std::ops::AddAssign for Complex {
	fn add_assign(&mut self, rhs: Self) {
		self.0 = self.0 + rhs.0;
		self.1 = self.1 + rhs.1;
	}
}

impl One for Complex {
	fn one() -> Self {
		Complex( 1.0,  0. )
	}
}

impl std::convert::From<i32> for Complex {
	fn from(value: i32) -> Self {
		Complex( value as f32,  0. )
	}
}

impl From<(f32, f32)> for Complex {
	fn from(value: (f32, f32)) -> Self {
		Complex(value.0, value.1)
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

impl<const L: usize> CanDoaDotProduct<Complex, L> for Complex {
	fn dot(first: &super::definition::vector<Complex, L>, other: super::definition::vector<Complex, L>) -> Complex {
		let mut result: Complex = Complex::default();
		for i in 0..L {
			unsafe {
				result = result + *first.0.get_unchecked(i) * other.0.get_unchecked(i).conjugate();
			}
		}
		result
	}
}