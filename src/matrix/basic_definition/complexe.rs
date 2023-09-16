use super::trait_definition::Complexe;

macro_rules! impl_Complexe {
	(for $($t:ty), +) => {
		$(impl Complexe for $t {
			fn conjugate(&self) -> Self {
				*self
			}
		})*
	};
}

impl_Complexe!(for u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64);