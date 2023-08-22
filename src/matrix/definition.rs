use std::ops::{Add, Sub, Mul};
use std::fmt;

// -------------------------- Basic implementation ------------------------

#[allow(dead_code)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Matrix<K: Scalar> {
    pub data: Vec::<K>,
    col: u8,
    row: u8,
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
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Clone + Sized + fmt::Display
{
}

pub trait Normable :
    std::iter::Sum + PartialOrd
{
    fn norm(&self) -> Self;
    fn square(&self) -> Self;
    fn square_root(&self) -> Self;
}

impl Normable for f64 {
    fn norm(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn square_root(&self) -> Self {
        self.sqrt()
    }
}

impl Normable for f32 {
    fn norm(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn square_root(&self) -> Self {
        self.sqrt()
    }
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

impl<K: Scalar> Add for Matrix<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col || self.data.len() != rhs.data.len() || self.data.len() == 0 {
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

impl<K: Scalar> From<Vec<K>> for Vector<K> {
    fn from(value: Vec<K>) -> Self {
        if value.len() == 0 {
            panic!("zero length vector creation error");
        }
        Vector { v: value }
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

impl<K: Scalar> IntoIterator for Matrix<K> {
    type Item = K;
    type IntoIter = std::vec::IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<K: Scalar> IntoIterator for Vector<K> {
    type Item = K;
    type IntoIter = std::vec::IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
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
    /// return a tuple (column, row)
    pub fn get_shape(&self) -> (u8, u8) {
        (self.col, self.row)
    }

    pub fn is_of_matching_dimension(&self, other: &Matrix<K>) -> bool {
        self.row == other.row && self.col == other.col && self.data.len() == other.data.len()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<K> {
        self.data.iter_mut()
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