use super::trait_definition::Normable;

macro_rules! impl_Normable {
	(for $($t:ty), +) => {
		$(impl Normable for $t {
			fn norm(&self) -> Self {
				self.abs()
			}

			fn square(&self) -> Self {
				self * self
			}

			fn square_root(&self) -> Self {
				self.sqrt()
			}
		})*
	};
}

impl_Normable!(for f32, f64);