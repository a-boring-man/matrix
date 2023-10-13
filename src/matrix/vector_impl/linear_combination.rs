use std::ops::{Add, Mul};

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{vector}};

pub fn _linear_combination
    <K : Default + Copy + Add<Output = K> + Mul<Output = K>, const L: usize, const N: usize>
    (vec: [vector<K, L>; N], coef: [K; N]) -> vector<K, L> {
        
    let mut result = vector::<K, L>::new();
    for i in 0..N {
        unsafe {
            result += *vec.get_unchecked(i) * *coef.get_unchecked(i);
        }
    }
    result
}