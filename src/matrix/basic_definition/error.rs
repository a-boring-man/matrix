use std::fmt::Display;

#[derive(Debug)]
pub enum MatrixError {
	Empty,
	IsNotSquare,
	IsSingular,
	InvalidFormat,
}

impl std::error::Error for MatrixError {}

impl Display for MatrixError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MatrixError::Empty => write!(f, "Matrix or Vector is empty"),
			MatrixError::InvalidFormat => write!(f, "Wrong format"),
			MatrixError::IsNotSquare => write!(f, "matrix is not square"),
			MatrixError::IsSingular => write!(f, "matrix is singular"),
		}
	}
}
