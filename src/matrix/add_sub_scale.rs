use std::fmt::Display;

use super::basic_definition::{trait_definition::Scalar, definition::{Matrix, Vector}};

impl<K : Scalar + Display> Matrix<K> 
{
    pub fn self_add(&mut self, m: &Matrix<K>) {
        if !self.is_of_matching_dimension(m) || self.data.len() == 0 {
            panic!("dimension error in matrix addition");}
        self.iter_mut().zip(m.iter()).for_each(|(a, b)| *a = *a + *b);
    }

    pub fn self_sub(&mut self, m: &Matrix<K>) {
        if !self.is_of_matching_dimension(m) || self.data.len() == 0 {
            panic!("dimension error in matrix substraction");}
        self.iter_mut().zip(m.iter()).for_each(|(a, b)| *a = *a - *b);
    }

    pub fn self_scale(&mut self, a: K) {
        self.iter_mut().for_each(|e| *e = e.clone() * a.clone());
    }

    pub fn add(&self, m: &Matrix<K>) -> Matrix<K> {
        if !self.is_of_matching_dimension(m) || self.data.len() == 0 {
            panic!("dimension error in matrix addition");}
        let (col, row, _) = self.get_shape();
        Matrix {
            data: self.iter().zip(m.iter()).map(|(a, b)| *a + *b).collect::<Vec<_>>(), 
            col,
            row
        }
    }

    pub fn sub(&self, m: &Matrix<K>) -> Matrix<K> {
        if !self.is_of_matching_dimension(m) || self.data.len() == 0 {
            panic!("dimension error in matrix substraction");}
        let (col, row, _) = self.get_shape();
        Matrix {
            data: self.iter().zip(m.iter()).map(|(a, b)| *a - *b).collect::<Vec<_>>(), 
            col, 
            row
        }
    }

    pub fn scale(&self, a: K) -> Matrix<K> {
        let (col, row, _) = self.get_shape();
        Matrix {
            data: self.iter().map(|e| e.clone() * a.clone()).collect::<Vec<_>>(), 
            col, 
            row
        }
    }
}

impl<K: Scalar> Vector<K>
{
    pub fn self_add(&mut self, v: &Vector<K>) {
        if self.v.len() != v.v.len() {
            panic!("dimension error on vector addition");}
        self.iter_mut().zip(v.iter()).for_each(|(a, b)| *a = *a + *b);
    }

    pub fn self_sub(&mut self, v: &Vector<K>) {
        if self.v.len() != v.v.len() {
            panic!("dimension error on vector substraction");}
            self.iter_mut().zip(v.iter()).for_each(|(a, b)| *a = *a - *b);
    }

    pub fn self_scale(&mut self, a: K) {
        self.iter_mut().for_each(|e| *e = e.clone() * a.clone());
    }

    pub fn add(&self, v: &Vector<K>) -> Vector<K> {
        if self.v.len() != v.v.len() || self.v.len() == 0 {
            panic!("dimension error on vector addition");}
        Vector {
            v: self.iter().zip(v.iter()).map(|(a, b)| *a + *b).collect::<Vec<_>>(),
        }
    }

    pub fn sub(&self, v: &Vector<K>) -> Vector<K> {
        if self.v.len() != v.v.len() {
            panic!("dimension error on vector substraction");}
        Vector {
            v: self.iter().zip(v.iter()).map(|(a, b)| a.clone() - b.clone()).collect::<Vec<_>>(),
        }
    }

    pub fn scale(&self, a: &K) -> Vector<K> {
        Vector {
            v: self.iter().map(|e| e.clone() * a.clone()).collect::<Vec<_>>(),
        } 
    }
}