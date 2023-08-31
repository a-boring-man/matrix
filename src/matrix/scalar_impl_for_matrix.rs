use std::ops::{Add, Sub, Mul};

use super::{trait_definition::Scalar, definition::Matrix};

// ----------------------------- Scalar traits impl ----------------------------

impl<K: Scalar> Add<Matrix<K>> for Matrix<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col || self.data.len() != rhs.data.len() || self.data.len() == 0 {
            panic!("dimension error in matrix addition");
        }
        Matrix {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
}
impl<K: Scalar> Add<Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;

    fn add(self, rhs: Matrix<K>) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col || self.data.len() != rhs.data.len() || self.data.len() == 0 {
            panic!("dimension error in matrix addition");
        }
        Matrix {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
}

impl<K: Scalar> Sub for Matrix<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col || self.data.len() != rhs.data.len() || self.data.len() == 0 {
            panic!("dimension error in matrix substraction");
        }
        Matrix {
            data: self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
}

impl<K: Scalar> Mul for Matrix<K> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self
    }
}

impl<K: Scalar> From<(Vec<K>, u8, u8)> for Matrix<K> {
    fn from(value: (Vec<K>, u8, u8)) -> Self {
        let (data, col, row) = value;
        if (col * row) as usize != data.len() || data.len() == 0 {
            panic!("invalid matrix constrution");
        }
        Matrix { data, col, row }
    }
}
