use super::{trait_definition::Scalar, definition::{Matrix, Vector, matrix, vector}, complex::Complex};

use core::fmt;

// ----------------------------------- Display function ---------------------------------

impl<K: Scalar + fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for r in 0..self.row {
            match r == 0 {
                true => write!(f, "[")?,
                false => write!(f, "  [")?,
            }
            for c in 0..self.col {
                match c != self.col - 1 {
                    true => write!(f, "{}, ", self.data[self.linear_index(c, r) as usize])?,
                    false => write!(f, "{}", self.data[self.linear_index(c, r) as usize])?,
                }
            }
            match r != self.row - 1 {
                true => writeln!(f, "]")?,
                false => write!(f, "]")?,
            }
        }
        writeln!(f, " ]")
    }
}

impl<K: fmt::Display, const R: usize, const C: usize> fmt::Display for matrix<K, R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for r in 0..R {
            match r == 0 {
                true => write!(f, "[")?,
                false => write!(f, "  [")?,
            }
            for c in 0..C {
                match c != C - 1 {
                    true => write!(f, "{}, ", self.e[r][c])?,
                    false => write!(f, "{}", self.e[r][c])?,
                }
            }
            match R == 1 {
                true => write!(f, "]")?,
                false => {
                    match r != R - 1 {
                        true => writeln!(f, "]")?,
                        false => write!(f, "]")?,
                    }
                },
            }
            
        }
        write!(f, " ]")
    }
}

impl<K: Scalar + fmt::Display> fmt::Display for Vector<K> {
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

impl<K: fmt::Display, const L: usize> fmt::Display for vector<K, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for index in 0..L {
            match index != L - 1 {
                true => write!(f, "{}, ", self.e[index])?,
                false => write!(f, "{}", self.e[index])?,
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Real: {}, Imaginary: {}", self.re, self.im)
    }
}