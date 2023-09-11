use std::fmt::Display;

use super::basic_definition::{trait_definition::Scalar, definition::{Matrix, Vector, matrix}};

impl<K : Scalar + Display> Matrix<K> {
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

impl<K: Scalar, const R: usize, const C: usize> matrix<K, R, C> {
    /// Accumulate the value of other into self
    pub fn self_add(&mut self, other: &matrix<K, R, C>) {
        self
            .iter_mut()
            .zip(other.iter())
            .for_each(|(vec1, vec2)| {
                vec1
                    .iter_mut()
                    .zip(vec2.iter())
                    .for_each(|(v1, v2)| *v1 = *v1 + *v2);
        })
    }

    // Return a matrix that contain all the element of self scaled by f
    // pub fn scale(&self, f: K) -> matrix<K, R, C> {
    //     type Output = Self;
    //     fn mul(self, coef: S) -> Self::Output {
    //         Matrix(self.0.map(|vec1| vec1.map(|v| v * coef)))
    //     }
    // }
}

impl<K: Scalar + Default, const R: usize, const C: usize> matrix<K, R, C> {
    /// Create a new matrix witch is the result of the addition of each value of the two input
    pub fn add(&self, other: &matrix<K, R, C>) -> matrix<K, R, C> {
        let mut e = [[K::default(); C]; R];
        for r in 0..R {
            for c in 0..C {
                unsafe {
                    *e.get_unchecked_mut(r).get_unchecked_mut(c) = 
                        *self.e.get_unchecked(r).get_unchecked(c) 
                        + *other.e.get_unchecked(r).get_unchecked(c);
                }
            }
        }
        matrix { e }
    }

    /// Create a new matrix witch is the result of the substraction of each value of the two input first minus second
    pub fn sub(&self, other: &matrix<K, R, C>) -> matrix<K, R, C> {
        let mut e = [[K::default(); C]; R];
        for r in 0..R {
            for c in 0..C {
                unsafe {
                    *e.get_unchecked_mut(r).get_unchecked_mut(c) = 
                        *self.e.get_unchecked(r).get_unchecked(c) 
                        - *other.e.get_unchecked(r).get_unchecked(c);
                }
            }
        }
        matrix { e }
    }
}

    // pub fn self_sub(&mut self, m: &Matrix<K>) {
    //     if !self.is_of_matching_dimension(m) || self.data.len() == 0 {
    //         panic!("dimension error in matrix substraction");}
    //     self.iter_mut().zip(m.iter()).for_each(|(a, b)| *a = *a - *b);
    // }

    // pub fn self_scale(&mut self, a: K) {
    //     self.iter_mut().for_each(|e| *e = e.clone() * a.clone());
    // }


    // pub fn sub(&self, m: &Matrix<K>) -> Matrix<K> {
    //     if !self.is_of_matching_dimension(m) || self.data.len() == 0 {
    //         panic!("dimension error in matrix substraction");}
    //     let (col, row, _) = self.get_shape();
    //     Matrix {
    //         data: self.iter().zip(m.iter()).map(|(a, b)| *a - *b).collect::<Vec<_>>(), 
    //         col, 
    //         row
    //     }
    // }

    

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