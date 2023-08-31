use super::{trait_definition::Scalar, definition::Vector};

use std::ops::{Add, Sub, Mul};

// ----------------------------- Scalar traits impl ----------------------------

impl<K: Scalar> Add for Vector<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.v.len() != rhs.v.len() || self.v.len() == 0 {
            panic!("dimension error on vector addition");
        }
        Vector {
            v: self.v.iter().zip(rhs.v.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
        }
    }
}

impl<K: Scalar> Sub for Vector<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.v.len() != rhs.v.len() || self.v.len() == 0 {
            panic!("dimension error on vector substraction");
        }
        Vector {
            v: self.v.iter().zip(rhs.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
        }
    }
}

impl<K: Scalar> Sub for &Vector<K> {
    type Output = Vector<K>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.v.len() != rhs.v.len() || self.v.len() == 0 {
            panic!("dimension error on vector substraction");
        }
        Vector {
            v: self.v.iter().zip(rhs.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
        }
    }
}

impl<K: Scalar> Mul for Vector<K> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.v.len() != rhs.v.len() || self.v.len() == 0 {
        panic!("dimension error on vector scaling");
    }
        self
    }
}

impl<K: Scalar> Mul for &Vector<K> {
    type Output = Vector<K>;
    
    fn mul(self, rhs: Self) -> Self::Output {
        if self.v.len() != rhs.v.len() || self.v.len() == 0 {
            panic!("dimension error on vector scaling");
        }
        self.clone()
    }
}

impl<K: Scalar> From<Vec<K>> for Vector<K> {
    fn from(value: Vec<K>) -> Self {
        if value.len() == 0 {
            panic!("zero length vector creation error");
        }
        Vector { v: value }
    }
}
