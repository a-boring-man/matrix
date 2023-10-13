use std::{fmt::Display, ops::{Add, Sub, Mul, AddAssign}};

use super::basic_definition::{trait_definition::Scalar, definition::{matrix, vector}};

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
        e.iter_mut().zip(self.iter()).for_each(|(erow, selfrow)| erow.iter_mut().zip(selfrow.iter()).for_each(|(e, s)| {
            *e = *s * rhs;
        }));
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