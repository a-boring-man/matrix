use super::definition::{matrix, vector};
use num_traits::identities::One;

// ------------------------------- Utils function --------------------------------

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