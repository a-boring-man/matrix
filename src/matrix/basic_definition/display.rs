use super::{trait_definition::Scalar, definition::{matrix, vector}, complex::Complex};

use core::fmt;

// ----------------------------------- Display function ---------------------------------

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
                    true => write!(f, "{}, ", self.0[r][c])?,
                    false => write!(f, "{}", self.0[r][c])?,
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

impl<K: fmt::Display, const L: usize> fmt::Display for vector<K, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for index in 0..L {
            match index != L - 1 {
                true => write!(f, "{}, ", self.0[index])?,
                false => write!(f, "{}", self.0[index])?,
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Real: {}, Imaginary: {}", self.0, self.1)
    }
}