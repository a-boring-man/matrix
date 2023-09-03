use super::{trait_definition::Scalar, definition::{Matrix, Vector}};

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
    pub fn get_shape(&self) -> (u8, u8, usize) {
        (self.col, self.row, self.data.len())
    }
    /// return true if two matrix are of the same dimension false other wise
    pub fn is_of_matching_dimension(&self, other: &Matrix<K>) -> bool {
        self.row == other.row && self.col == other.col && self.data.len() == other.data.len()
    }
    /// return the length of the vector containing the data
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// return a identity matrix of type i32 and of size size
    pub fn identity(size: u8) -> Matrix<i32> {
        if size < 1 {
            panic!("invalide format");
        }
        let mut vec = Vec::with_capacity(size.pow(2) as usize);
        for r in 0..size {
            for c in 0..size {
                if c == r {
                    vec.push(1);
                }
                else {
                    vec.push(0);
                }
            }
        }
        Matrix { data: vec, col: size, row: size }
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
