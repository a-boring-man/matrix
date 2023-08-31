use super::{trait_definition::Scalar, definition::{Matrix, Vector}};

// -------------------------------- Iterator --------------------------------

// -------------------------------- Definition -----------------------------

pub struct MatrixIterator<'a, K: Scalar> {
    matrix: &'a Matrix<K>,
    current_row: u8,
    current_col: u8,
}

pub struct VectorIterator<'a, K: Scalar> {
    vector: &'a Vector<K>,
    current_index: usize,
}

// --------------------------------- Implementation --------------------------

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