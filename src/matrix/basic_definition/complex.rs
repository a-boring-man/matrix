use num_traits::Pow;
use num_traits::One;
use super::trait_definition::Zero;
use super::trait_definition::{Complexe, Normable};

#[derive(Copy, Clone, PartialEq, Debug, Default, PartialOrd)]
pub struct Complex (pub f64,pub f64);

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
impl std::ops::Neg for Complex {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Complex(-self.0, -self.1)
	}
}

impl One for Complex {
	fn one() -> Self {
		Complex( 1.0,  0. )
	}
}
impl Zero for Complex {
	fn close_to_zero(&self) -> bool {
		(self.0 - <f64>::default()).abs() < 1e-6 && (self.1 - <f64>::default()).abs() < 1e-6
	}
}