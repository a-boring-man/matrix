use std::ops::{Add, Sub, Mul};
use std::fmt;

// -------------------------- Basic implementation ------------------------

#[allow(dead_code)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Matrix<K: Scalar> {
    pub data: Vec::<K>,
    pub col: u8,
    pub row: u8,
}

#[allow(dead_code)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Vector<K: Scalar> {
    pub v: Vec::<K>,
}

// ---------------------------- Scalar trait definition --------------------------

pub trait Scalar :
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Clone + fmt::Display
{
}

impl<T> Scalar for T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone + fmt::Display
{
}

// ----------------------------- Scalar traits impl ----------------------------

impl<K: Scalar> Add for Vector<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.v.len() != rhs.v.len() {
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
        if self.v.len() != rhs.v.len() {
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
        self
    }
}

impl<K: Scalar> Add for Matrix<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col || self.data.len() != rhs.data.len() {
            panic!("dimension error in matrix addition");
        }
        Matrix {
            data: self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
            col: self.col,
            row: self.row,
        }
    }
}

impl<K: Scalar> Sub for Matrix<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col || self.data.len() != rhs.data.len() {
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

// ------------------------------- Utils function --------------------------------

#[allow(dead_code)]
#[allow(non_snake_case)]
impl<K: Scalar> Matrix<K> {
    pub fn is_square(&self) -> bool {
        self.col == self.row
    }

    pub fn transform_into_Vector(&self) -> Vector<K> {
        Vector { v: self.data.clone() }
    }

    pub fn linear_index(&self, r: u8, c: u8) -> u16 {
        (r * self.col + c).into()
    }
}

#[allow(dead_code)]
impl<K: Scalar> Vector<K> {
    pub fn transform_into_matrix(&self, linear: bool) -> Matrix<K> {
        match linear {
            true => Matrix {data: self.v.clone(), col: self.v.len() as u8 , row: 1},
            false => Matrix {data: self.v.clone(), col: 1, row: self.v.len() as u8 }
        }
    }
}

// ----------------------------------- Display function ---------------------------------

impl<K: Scalar> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for r in 0..self.row {
            match r == 0 {
                true => write!(f, "[")?,
                false => write!(f, "  [")?,
            }
            for c in 0..self.col {
                match c != self.col - 1 {
                    true => write!(f, "{}, ", self.data[self.linear_index(r, c) as usize])?,
                    false => write!(f, "{}", self.data[self.linear_index(r, c) as usize])?,
                }
            }
            match r != self.col - 1 {
                true => writeln!(f, "]")?,
                false => write!(f, "]")?,
            }
        }
        writeln!(f, " ]")
    }
}

impl<K: Scalar> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let size = self.v.len();
        for i in 0..size {
            match i != size - 1 {
                true => write!(f, "{}, ", self.v[i])?,
                false => write!(f, "{}", self.v[i])?,
            }
        }
        write!(f, "]")
    }
}