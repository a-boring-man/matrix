#[cfg(test)]
mod tests {
    use crate::matrix::basic_definition::definition::{Matrix, Vector};

    #[test]
    fn matrix_printing_test() {
        let test_u32: Matrix<u32> = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        println!("{}", test_u32);
        let test_i32: Matrix<i32> = Matrix::from((vec![1, 2, -3, 4], 2, 2));
        println!("{}", test_i32);
        let test_f64: Matrix<f64> = Matrix::from((vec![1.86956, 2.555, 3.976, 4.0008], 2, 2));
        println!("{}", test_f64);
    }

    #[test]
    fn vector_printing_test() {
        let test_m: Vector<u32> = Vector { v: vec![1, 2, 3, 4]};
        println!("{}", test_m);
        let test_mi: Vector<i32> = Vector { v: vec![1, -2, 3, 4]};
        println!("{}", test_mi);
        let test_mf: Vector<f32> = Vector { v: vec![1.22, -2.53, 0.3, 0.434]};
        println!("{}", test_mf);
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
        assert_eq!(m1.linear_index(0, 1), 1);
        assert_eq!(m1.linear_index(0, 2), 2);
        assert_eq!(m1.linear_index(1, 0), 3);
        assert_eq!(m1.linear_index(1, 1), 4);
        assert_eq!(m1.linear_index(1, 2), 5);
        assert_eq!(m1.linear_index(2, 0), 6);
        assert_eq!(m1.linear_index(2, 1), 7);
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