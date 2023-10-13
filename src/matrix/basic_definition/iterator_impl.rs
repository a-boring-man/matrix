use super::definition::{matrix, vector};

// -------------------------------- Iterator --------------------------------

impl<K, const R: usize, const C: usize> matrix<K, R, C> {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<[K; C]> {
        self.0.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<[K; C]> {
        self.0.iter()
    }
}

impl<K, const L: usize> vector<K, L> {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<K> {
        self.0.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<K> {
        self.0.iter()
    }
}