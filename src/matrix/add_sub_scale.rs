use super::definition::{Scalar, Matrix, Vector};

#[allow(dead_code)]
impl<K : Scalar> Matrix<K>
{
    fn self_add(&mut self, m: &Matrix<K>) {
        self.data = self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect();
    }
    fn self_sub(&mut self, m: &Matrix<K>) {
        self.data = self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect();
    }
    fn self_scale(&mut self, a: K) {
        let tmp: Vec<K> = self.data.iter().map(|e| e.clone() * a.clone()).collect();
        self.data = tmp;
    }
    pub fn add(&self, m: &Matrix<K>) -> Matrix<K> {
        Matrix {
            data: self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
    pub fn sub(&self, m: &Matrix<K>) -> Matrix<K> {
        Matrix {
            data: self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
    pub fn scale(&self, a: K) -> Matrix<K> {
        Matrix {
            data: self.data.iter().map(|e| e.clone() * a.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
}

#[allow(dead_code)]
impl<K: Scalar> Vector<K>
{
    fn self_add(&mut self, v: Vector<K>) {
        self.v = self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() + b.clone()).collect();
    }
    fn self_sub(&mut self, v: Vector<K>) {
        self.v = self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect();
    }
    fn self_scale(&mut self, a: K) {
        let tmp: Vec<K> = self.v.iter().map(|e| e.clone() * a.clone()).collect();
        self.v = tmp;
    }
    pub fn add(&mut self, v: Vector<K>) -> Vector<K> {
        if self.v.len() != v.v.len() {
            panic!("");
        }
        Vector {
            v: self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
        }
    }
    pub fn sub(&mut self, v: Vector<K>) -> Vector<K> {
        Vector {
            v: self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
        }
    }
    pub fn scale(&mut self, a: K) -> Vector<K> {
        Vector {
            v: self.v.iter().map(|e| e.clone() * a.clone()).collect(),
        }   
    }
}