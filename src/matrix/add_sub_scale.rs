use super::definition::{Scalar, Matrix, Vector};

#[allow(dead_code)]
impl<K : Scalar> Matrix<K>
{
    pub fn self_add(&mut self, m: &Matrix<K>) {
        if !self.is_of_matching_dimension(m) {
            panic!("dimension error in matrix addition");}
        self.data = self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect();
    }

    pub fn self_sub(&mut self, m: &Matrix<K>) {
        if !self.is_of_matching_dimension(m) {
            panic!("dimension error in matrix substraction");}
        self.data = self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect();
    }

    pub fn self_scale(&mut self, a: &K) {
        let tmp: Vec<K> = self.data.iter().map(|e| e.clone() * a.clone()).collect();
        self.data = tmp;
    }

    pub fn add(&self, m: &Matrix<K>) -> Matrix<K> {
        if !self.is_of_matching_dimension(m) {
            panic!("dimension error in matrix addition");}
        let (col, row) = self.get_shape();
        Matrix::from((
            self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect(), 
            col, 
            row))
    }

    pub fn sub(&self, m: &Matrix<K>) -> Matrix<K> {
        if !self.is_of_matching_dimension(m) {
            panic!("dimension error in matrix substraction");}
        let (col, row) = self.get_shape();
        Matrix::from((
            self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect(), 
            col, 
            row))
    }

    pub fn scale(&self, a: &K) -> Matrix<K> {
        let (col, row) = self.get_shape();
        Matrix::from((
            self.data.iter().map(|e| e.clone() * a.clone()).collect(), 
            col, 
            row))
    }
}

#[allow(dead_code)]
impl<K: Scalar> Vector<K>
{
    pub fn self_add(&mut self, v: &Vector<K>) {
        if self.v.len() != v.v.len() {
            panic!("dimension error on vector addition");}
        self.v = self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() + b.clone()).collect();
    }

    pub fn self_sub(&mut self, v: &Vector<K>) {
        if self.v.len() != v.v.len() {
            panic!("dimension error on vector substraction");}
        self.v = self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect();
    }

    pub fn self_scale(&mut self, a: &K) {
        let tmp: Vec<K> = self.v.iter().map(|e| e.clone() * a.clone()).collect();
        self.v = tmp;
    }

    pub fn add(&self, v: &Vector<K>) -> Vector<K> {
        if self.v.len() != v.v.len() {
            panic!("dimension error on vector addition");}
        Vector {
            v: self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
        }
    }

    pub fn sub(&self, v: &Vector<K>) -> Vector<K> {
        if self.v.len() != v.v.len() {
            panic!("dimension error on vector substraction");}
        Vector {
            v: self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
        }
    }

    pub fn scale(&self, a: &K) -> Vector<K> {
        Vector {
            v: self.v.iter().map(|e| e.clone() * a.clone()).collect(),
        }   
    }
}