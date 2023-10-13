use super::trait_definition::Zero;

macro_rules! impl_zero {
	(for $($t:ty), +) => {
		$(impl Zero for $t {
			fn close_to_zero(&self) -> bool {
				*self == <$t>::default()
			}
		})*
	};
}

impl_zero!(for u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

macro_rules! impl_close_zero {
	(for $($t:ty), +) => {
		$(impl Zero for $t {
			fn close_to_zero(&self) -> bool {
				(*self - <$t>::default()).abs() < 1e-6
			}
		})*
	};
}

impl_close_zero!(for f64, f32);