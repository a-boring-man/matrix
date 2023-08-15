use std::ops::{Mul, Add, Sub};

#[allow(dead_code)]
struct Matrix<K> {
    data: Vec::<K>,
    col: u8,
    row: u8,
}

#[allow(dead_code)]
impl<K> Matrix<K>
where
    K: Add<Output = K> + Mul<Output = K> + Sub<Output = K> + Clone,
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
    fn add(&self, m: &Matrix<K>) -> Matrix<K> {
        Matrix {
            data: self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
    fn sub(&self, m: &Matrix<K>) -> Matrix<K> {
        Matrix {
            data: self.data.iter().zip(m.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
    fn scale(&self, a: K) -> Matrix<K> {
        Matrix {
            data: self.data.iter().map(|e| e.clone() * a.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
}