use super::definition::*;

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