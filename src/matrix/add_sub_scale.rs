use std::{fmt::Display, ops::{Add, Sub, Mul, AddAssign}};

use super::basic_definition::{trait_definition::Scalar, definition::{Matrix, Vector, matrix, vector}};

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

impl<K: Scalar> Vector<K> {
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

impl<K: Add<Output = K> + Copy, const R: usize, const C: usize> AddAssign for matrix<K, R, C> {
    
    fn add_assign(&mut self, rhs: Self) {
        self
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(vec1, vec2)| {
                vec1
                    .iter_mut()
                    .zip(vec2.iter())
                    .for_each(|(v1, v2)| *v1 = *v1 + *v2);
        })
    }
}
impl<K: Default + Copy + Add<Output = K>, const R: usize, const C: usize> Add for matrix<K, R, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut e = [[K::default(); C]; R];
        for r in 0..R {
            for c in 0..C {
                unsafe {
                    *e.get_unchecked_mut(r).get_unchecked_mut(c) = 
                        *self.0.get_unchecked(r).get_unchecked(c) 
                        + *rhs.0.get_unchecked(r).get_unchecked(c);
                }
            }
        }
        matrix(e)
    }
}
impl<K: Default + Copy + Sub<Output = K>, const R: usize, const C: usize> Sub for matrix<K, R, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut e = [[K::default(); C]; R];
        for r in 0..R {
            for c in 0..C {
                unsafe {
                    *e.get_unchecked_mut(r).get_unchecked_mut(c) = 
                        *self.0.get_unchecked(r).get_unchecked(c) 
                        - *rhs.0.get_unchecked(r).get_unchecked(c);
                }
            }
        }
        matrix(e)
    }
}
impl<K: Default + Copy + Mul<Output = K>, const R: usize, const C: usize> Mul<K> for matrix<K, R, C> {
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        let mut e = [[K::default(); C]; R];
        e.iter_mut().zip(self.iter()).for_each(|(erow, selfrow)| erow.iter_mut().zip(selfrow.iter()).for_each(|(e, s)| *e = *s * rhs));
        matrix(e)
    }
}

impl<K: Add<Output = K> + Copy, const L: usize> AddAssign for vector<K, L> {
    fn add_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(v1, v2)| *v1 = *v1 + *v2);
    }
}
impl<K: Default + Copy + Add<Output = K>, const L: usize> Add for vector<K, L> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut e = [K::default(); L];
        for i in 0..L {
            unsafe {
                *e.get_unchecked_mut(i) = *self.0.get_unchecked(i) + *rhs.0.get_unchecked(i);
            }
        }
        vector(e)
    }
}
impl<K: Default + Copy + Sub<Output = K>, const L: usize> Sub for vector<K, L> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut e = [K::default(); L];
        for i in 0..L {
            unsafe {
                *e.get_unchecked_mut(i) = *self.0.get_unchecked(i) - *rhs.0.get_unchecked(i);
            }
        }
        vector(e)
    }
}
impl<K: Default + Copy + Mul<Output = K>, const L: usize> Mul<K> for vector<K, L> {
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        let mut e = [K::default(); L];
        e.iter_mut().zip(self.iter()).for_each(|(e, s)| *e = *s * rhs);
        vector(e)
    }
}