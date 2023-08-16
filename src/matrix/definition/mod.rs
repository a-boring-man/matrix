use std::ops::{Add, Sub, Mul};
use std::fmt;

#[allow(dead_code)]
pub struct Matrix<K> {
    pub data: Vec::<K>,
    pub col: u8,
    pub row: u8,
}

#[allow(dead_code)]
pub struct Vector<K> {
    pub v: Vec::<K>,
}

pub trait Scalar :
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Clone + fmt::Display
{
}

impl<T> Scalar for T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone + fmt::Display
{
}

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

impl<K: Scalar> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for r in 0..self.row {
            write!(f, "[")?;
            for c in 0..self.col {
                let li = self.linear_index(r, c);
                write!(f, "{}, ", self.data[li as usize])?;
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "]")
    }
}

impl<K: Scalar> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let size = self.v.len();
        for i in 0..size {
            write!(f, "{}, ", self.v[i])?;
        }
        writeln!(f, "]")
    }
}