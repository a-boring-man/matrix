use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::path::Iter;

// -------------------------- Basic implementation ------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<K: Scalar> {
    pub(in crate) data: Vec::<K>,
    pub(in crate) col: u8,
    pub(in crate) row: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<K: Scalar> {
    pub(in crate) v: Vec::<K>,
}

pub struct MatrixIterator<'a, K: Scalar> {
    matrix: &'a Matrix<K>,
    current_row: u8,
    current_col: u8,
}

pub struct VectorIterator<'a, K: Scalar> {
    vector: &'a Vector<K>,
    current_index: usize,
}

// ---------------------------- Scalar trait definition --------------------------

pub trait Scalar :
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Clone + Sized
    + fmt::Display
    // where 
    // for<'a> &'a mut Self: Add<&'a Self>,
    // for<'a> &'a mut Self: Sub<&'a Self>,
    // for<'a> &'a mut Self: Mul<&'a Self>,
    // for<'a> &'a Self: Add<&'a Self, Output = Self>,
    // for<'a> &'a Self: Sub<&'a Self, Output = Self>,
    // for<'a> &'a Self: Mul<&'a Self, Output = Self>
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
    T: Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Clone
    + fmt::Display,
    // for<'a> &'a mut Self: Add<&'a Self>,
    // for<'a> &'a mut Self: Sub<&'a Self>,
    // for<'a> &'a mut Self: Mul<&'a Self>,
    // for<'a> &'a Self: Add<&'a Self, Output = Self>,
    // for<'a> &'a Self: Sub<&'a Self, Output = Self>,
    // for<'a> &'a Self: Mul<&'a Self, Output = Self>
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

impl<K: Scalar> From<Vec<K>> for Vector<K> {
    fn from(value: Vec<K>) -> Self {
        if value.len() == 0 {
            panic!("zero length vector creation error");
        }
        Vector { v: value }
    }
}

// -------------------------------- Iterator --------------------------------

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

impl<'a, K: Scalar> MatrixIterator<'a, K> {
    pub fn new(matrix: &'a Matrix<K>) ->Self {
        MatrixIterator {
            matrix,
            current_row: 0,
            current_col: 0,
        }
    }
}

impl<'a, K: Scalar> VectorIterator<'a, K> {
    pub fn new(vector: &'a Vector<K>) -> Self {
        VectorIterator {
            vector,
            current_index: 0,
        }
    }
}

impl<'a, K: Scalar> Iterator for MatrixIterator<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.matrix.row {
            let index = (self.current_row * self.matrix.col as u8 + self.current_row) as usize;
            let element = &self.matrix.data[index];

            self.current_col += 1;
            if self.current_col >= self.matrix.col {
                self.current_col = 0;
                self.current_row += 1;
            }

            Some(element)
        }
        else {
            None
        }
    }
}

impl<'a, K: Scalar> Iterator for VectorIterator<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.vector.len() {
            let index = self.current_index;
            let element = &self.vector.v[index];

            self.current_index += 1;

            Some(element)
        }
        else {
            None
        }
    }
}

impl<K: Scalar> Matrix<K> {
    
    pub fn iter_mut(&mut self) -> std::slice::IterMut<K> {
        self.data.iter_mut()
    }

    pub fn iter(&self) -> MatrixIterator<'_, K> {
        MatrixIterator::new(self)
    }
}

impl<K: Scalar> Vector<K> {

    pub fn iter_mut(&mut self) -> std::slice::IterMut<K> {
        self.v.iter_mut()
    }

    pub fn iter(&self) -> VectorIterator<'_, K> {
        VectorIterator::new(self)
    }
}

// ------------------------------- Utils function --------------------------------

#[allow(non_snake_case)]
impl<K: Scalar> Matrix<K> {
    /// return true if the matrix is square false otherwise
    pub fn is_square(&self) -> bool {
        self.col == self.row
    }
    /// transform a matrix into a vector containing the same number of element
    pub fn transform_into_Vector(&self) -> Vector<K> {
        Vector { v: self.data.clone() }
    }
    /// given a column and a row in this order determin the linear index use to acces the underlying data
    pub fn linear_index(&self, c: u8, r: u8) -> u16 {
        r as u16 * self.col as u16 + c as u16
    }
    /// return a tuple (column, row)
    pub fn get_shape(&self) -> (u8, u8) {
        (self.col, self.row)
    }
    /// return true if two matrix are of the same dimension false other wise
    pub fn is_of_matching_dimension(&self, other: &Matrix<K>) -> bool {
        self.row == other.row && self.col == other.col && self.data.len() == other.data.len()
    }
    /// return the length of the vector containing the data
    pub fn len(&self) -> usize {
        self.data.len()
    }

}

impl<K: Scalar> Vector<K> {
    /// transform a vector into a matrix with an linear flag for a linear matrix transformation
    pub fn transform_into_matrix(&self, linear: bool) -> Matrix<K> {
        if self.v.len() == 0 {
            panic!("no dimension in vector to matrix conversion");
        }
        match linear {
            true => Matrix {data: self.v.clone(), col: self.v.len() as u8 , row: 1},
            false => Matrix {data: self.v.clone(), col: 1, row: self.v.len() as u8 }
        }
    }
    /// return the length of the underlying vector
    pub fn len(&self) -> usize {
        self.v.len()
    }
    /// a function to get a reference to the element at index index inside the vector
    pub fn get_elm(&self, index: usize) -> &K {
        self.v.get(index).unwrap()
    }
    /// a function to get a mutable reference to the element at index index inside the vector
    pub fn get_elm_mut(&mut self, index: usize) -> &mut K {
        self.v.get_mut(index).unwrap()
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