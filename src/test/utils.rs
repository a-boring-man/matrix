#[cfg(test)]
mod tests {
    use crate::matrix::basic_definition::{definition::{Matrix, Vector, matrix, vector}, complex::Complex};

    #[test]
    fn matrix_printing_test() {
        println!("{}\n", matrix([[1, 2], [3, 4]]));
        println!("{}\n", matrix([[1., 2., 3.], [4., -5., 6.], [7., 8., 9.]]));
        println!("{}\n", matrix([[Complex(1., 2.), Complex(3., 4.)]]));
    }

    #[test]
    fn vector_printing_test() {
        println!("{}\n", vector([1, 2, 3, 4]));
        println!("{}\n", vector([1., 2., 3., -4., 5., 6., 7., 8., 9.]));
        println!("{}\n", vector([Complex(4., 5.), Complex(3., 4.2)]));
    }

    #[test]
    fn identity() {
        assert_eq!(matrix::<i32, 2, 2>::identity(), matrix([[1, 0], [0, 1]]));
        assert_eq!(matrix::<f32, 3, 3>::identity(), matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]));
        assert_eq!(matrix::<Complex, 2, 2>::identity(), matrix([[Complex(1., 0.), Complex(0., 0.)], [Complex(0., 0.), Complex(1., 0.)]]));
    }

    #[test]
    fn new() {
        assert_eq!(matrix::<i32, 3, 2>::new(), matrix([[0, 0], [0, 0], [0, 0]]));
        assert_eq!(matrix::<Complex, 1, 2>::new(), matrix([[Complex(0., 0.), Complex(0., 0.)]]));

        assert_eq!(vector::<i32, 2>::new(), vector([0, 0]));
        assert_eq!(vector::<Complex, 2>::new(), vector([Complex(0., 0.), Complex(0., 0.)]));
    }

    #[test]
    fn isolate_column_vector() {
        assert_eq!(matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]).isolate_column_vector(2), vector([3, 6, 9]));
    }

    #[test]
    fn from() {
        let salut = [[1, 2], [3, 4]];
        assert_eq!(matrix([[1, 2], [3, 4]]), matrix::from(salut));
    }

    #[test]
    fn matrix_square_test() {
        let m_not_squar1: Matrix<f64> = Matrix::from((vec![2.43, 234.35, 235.663], 3, 1));
        let m_not_squar2: Matrix<f64> = Matrix::from((vec![2.43, 234.35, 235.663, 35.263, 36.0, 763.2], 2, 3));
        let m_squar: Matrix<f64> = Matrix::from((vec![2.43, 234.35, 235.663, 42.0], 2, 2));
        assert!(!m_not_squar1.is_square());
        assert!(!m_not_squar2.is_square());
        assert!(m_squar.is_square());
    }

    #[test]
    fn transform_into_vector() {
        let m1: Matrix<f64> = Matrix::from((vec![1.0, 2.312, 3.4124, 4.51], 2, 2));
        let v1: Vector<f64> = Vector { v: vec![1.0, 2.312, 3.4124, 4.51] };
        assert_eq!(m1.transform_into_Vector(), v1);
    }

    #[test]
    fn linear_index() {
        let m1 = Matrix::from((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3));
        assert_eq!(m1.linear_index(0, 0), 0);
        assert_eq!(m1.linear_index(1, 0), 1);
        assert_eq!(m1.linear_index(2, 0), 2);
        assert_eq!(m1.linear_index(0, 1), 3);
        assert_eq!(m1.linear_index(1, 1), 4);
        assert_eq!(m1.linear_index(2, 1), 5);
        assert_eq!(m1.linear_index(0, 2), 6);
        assert_eq!(m1.linear_index(1, 2), 7);
        assert_eq!(m1.linear_index(2, 2), 8);
    }

    #[test]
    fn transform_into_matrix() {
        let v = Vector {v: vec![1, 2, 3, 4, 5, 6, 7, 8]};
        let ml = Matrix::from((vec![1, 2, 3, 4, 5, 6, 7, 8], 8, 1));
        let mnl = Matrix::from((vec![1, 2, 3, 4, 5, 6, 7, 8], 1, 8));
        assert_eq!(v.transform_into_matrix(true), ml);
        assert_eq!(v.transform_into_matrix(false), mnl);
    }

}