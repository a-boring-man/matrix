use super::{trait_definition::{Scalar, CanDoaDotProduct}, definition::{Matrix, Vector, matrix, vector}};
use num_traits::identities::One;

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

impl<K: Scalar> From<(Vec<K>, u8, u8)> for Matrix<K> {
    fn from(value: (Vec<K>, u8, u8)) -> Self {
        let (data, col, row) = value;
        if (col * row) as usize != data.len() || data.len() == 0 {
            panic!("invalid matrix constrution");
        }
        Matrix { data, col, row }
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

impl<K: Scalar> From<Vec<K>> for Vector<K> {
    fn from(value: Vec<K>) -> Self {
        if value.len() == 0 {
            panic!("zero length vector creation error");
        }
        Vector { v: value }
    }
}

impl<K: Copy + One + Default, const R: usize> matrix<K, R, R> {
    /// return a identity matrix of type i32 and of size size
    pub fn identity() -> Self {
        let mut m = matrix([[K::default(); R]; R]);
        for r in 0..R {
			for c in 0..R {
				if r == c {
					m.0[r][c] = K::one();
				}
			}
		}
        m
    }
}

impl<K: Default + Copy, const R: usize, const C: usize> matrix<K, R, C> {
    /// Return a new matrix with K::default value
    pub fn new() -> Self {
        matrix([[K::default(); C]; R])
    }

    /// Return the column vector of the specified column
    pub fn isolate_column_vector(&self, column: usize) -> vector<K, R> {
        if column >= C {
            panic!("column index is greater than Matrix max column");
        }
        let mut result = vector::<K, R>::new();
        for r in 0..R {
            result.0[r] = self.0[r][column];
        }
        result
    }
}

impl<K: Default + Copy, const L: usize> vector<K, L> {
    /// Return a new vector with K::default value
    pub fn new() -> Self {
        vector([K::default(); L])
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for matrix<K, R, C> {
    fn from(value: [[K; C]; R]) -> Self {
        matrix(value)
    }
}

impl<K, const L: usize> From<[K; L]> for vector<K, L> {
    fn from(value: [K; L]) -> Self {
        vector(value)
    }
}