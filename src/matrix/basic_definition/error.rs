use std::fmt::{Display, write};

#[derive(Debug)]
pub enum MatrixError {
	Empty,
	IsNotSquare,
	IsSingular,
	InvalidFormat,
	NotInversible,
}

#[derive(Debug)]
pub enum matrixError {
	IsSingular,
	NotInversible,
}

impl std::error::Error for matrixError {}

impl Display for matrixError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			matrixError::IsSingular => write!(f, "matrix is singular"),
			matrixError::NotInversible => write!(f, "matrix is not inversible"),
		}
	}
}

impl std::error::Error for MatrixError {}

impl Display for MatrixError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MatrixError::Empty => write!(f, "Matrix or Vector is empty"),
			MatrixError::InvalidFormat => write!(f, "Wrong format"),
			MatrixError::IsNotSquare => write!(f, "matrix is not square"),
			MatrixError::IsSingular => write!(f, "matrix is singular"),
			MatrixError::NotInversible => write!(f, "matrix is not inversible"),
		}
	}
}
