use super::definition::{Matrix, Vector};
use num_traits::identities::One;

// ------------------------------- Utils function --------------------------------

impl<K: Copy + One + Default, const R: usize> Matrix<K, R, R> {
    /// return a identity matrix of type i32 and of size size
    pub fn identity() -> Self {
        let mut m = Matrix([[K::default(); R]; R]);
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

impl<K: Default + Copy, const R: usize, const C: usize> Default for Matrix<K, R, C> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Default + Copy, const R: usize, const C: usize> Matrix<K, R, C> {
    /// Return a new matrix with K::default value
    pub fn new() -> Self {
        Matrix([[K::default(); C]; R])
    }

    /// Return the column vector of the specified column
    pub fn isolate_column_vector(&self, column: usize) -> Vector<K, R> {
        if column >= C {
            panic!("column index is greater than Matrix max column");
        }
        let mut result = Vector::<K, R>::new();
        for r in 0..R {
            result.0[r] = self.0[r][column];
        }
        result
    }
}

impl<K: Default + Copy, const L: usize> Default for Vector<K, L> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Default + Copy, const L: usize> Vector<K, L> {
    /// Return a new vector with K::default value
    pub fn new() -> Self {
        Vector([K::default(); L])
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K, R, C> {
    fn from(value: [[K; C]; R]) -> Self {
        Matrix(value)
    }
}

impl<K, const L: usize> From<[K; L]> for Vector<K, L> {
    fn from(value: [K; L]) -> Self {
        Vector(value)
    }
}