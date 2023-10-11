use std::ops::{Add, Mul};

use crate::matrix::basic_definition::{trait_definition::Scalar, definition::{Vector, vector}};

impl<K: Scalar> Vector<K> {
    pub fn linear_combination(u : &[Vector<K>], coefs: &[K]) -> Vector<K> {
        let number_of_vector = u.len();
        if !(number_of_vector > 0 && number_of_vector == coefs.len()) {
            panic!("wrong dimension in linear combination");
        }
        let mut tmp_vec = Vector::from(u[0].scale(&coefs[0]));
        for i in 1..number_of_vector {
            tmp_vec.self_add(&u[i].scale(&coefs[i]));
        }
        return tmp_vec;
    }
}

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