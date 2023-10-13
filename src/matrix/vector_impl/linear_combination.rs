use std::ops::{Add, Mul};

use crate::matrix::basic_definition::definition::Vector;

pub fn _linear_combination
    <K : Default + Copy + Add<Output = K> + Mul<Output = K>, const L: usize, const N: usize>
    (vec: [Vector<K, L>; N], coef: [K; N]) -> Vector<K, L> {
        
    let mut result = Vector::<K, L>::new();
    for i in 0..N {
        unsafe {
            result += *vec.get_unchecked(i) * *coef.get_unchecked(i);
        }
    }
    result
}