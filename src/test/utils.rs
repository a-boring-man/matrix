#[cfg(test)]
mod tests {
    
    use crate::matrix::definition::*;

    #[test]
    fn matrix_printing_test() {
        let test_u32: Matrix<u32> = Matrix {data: vec![1, 2, 3, 4], row: 2, col: 2};
        println!("{}", test_u32);
        let test_i32: Matrix<i32> = Matrix {data: vec![1, 2, -3, 4], row: 2, col: 2};
        println!("{}", test_i32);
        let test_f64: Matrix<f64> = Matrix {data: vec![1.86956, 2.555, 3.976, 4.0008], row: 2, col: 2};
        println!("{}", test_f64);
        let test_vector_u32: Matrix<Vector<u32>> = Matrix {data: vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}, Vector {v: vec![9, 10, 11, 12]}, Vector {v: vec![13, 14, 15, 16]}], row: 2, col: 2};
        println!("{}", test_vector_u32);
        let ultimate: Matrix<Vector<Matrix<i32>>> = Matrix {data: vec![Vector {v: vec![Matrix {data: vec![-1, 2, 3, 4], row: 2, col: 2}, Matrix {data: vec![1, -2, 3, 4], row: 2, col: 2}, Matrix {data: vec![1, 2, -3, 4], row: 2, col: 2}, Matrix {data: vec![1, 2, 3, -4], row: 2, col: 2}]}, Vector {v: vec![Matrix {data: vec![-1, 2, 3, 4], row: 2, col: 2}, Matrix {data: vec![-1, -2, 3, 4], row: 2, col: 2}, Matrix {data: vec![-1, 2, -3, 4], row: 2, col: 2}, Matrix {data: vec![-1, 2, 3, -4], row: 2, col: 2}]}, Vector {v: vec![Matrix {data: vec![-1, -2, 3, 4], row: 2, col: 2}, Matrix {data: vec![1, -2, -3, 4], row: 2, col: 2}, Matrix {data: vec![1, 2, -3, 4], row: 2, col: 2}, Matrix {data: vec![1, 2, -3, -4], row: 2, col: 2}]}, Vector {v: vec![Matrix {data: vec![-1, 2, 3, -4], row: 2, col: 2}, Matrix {data: vec![1, -2, 3, -4], row: 2, col: 2}, Matrix {data: vec![1, 2, -3, -4], row: 2, col: 2}, Matrix {data: vec![1, 2, 3, -4], row: 2, col: 2}]}], row: 2, col: 2};
        println!("{}", ultimate);
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
        let m_not_squar1: Matrix<f64> = Matrix {data: vec![2.43, 234.35, 235.663], col: 3, row: 1};
        let m_not_squar2: Matrix<f64> = Matrix {data: vec![2.43, 234.35, 235.663, 35.263, 36.0, 763.2], col: 2, row: 3};
        let m_squar: Matrix<f64> = Matrix {data: vec![2.43, 234.35, 235.663, 42.0], col: 2, row: 2};
        assert!(!m_not_squar1.is_square());
        assert!(!m_not_squar2.is_square());
        assert!(m_squar.is_square());
    }

    #[test]
    fn transform_into_vector() {
        let m1: Matrix<f64> = Matrix { data: vec![1.0, 2.312, 3.4124, 4.51], col: 2, row: 2 };
        let m2: Matrix<Vector<u32>> = Matrix { data: vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}], col: 2, row: 1 };
        let v1: Vector<f64> = Vector { v: vec![1.0, 2.312, 3.4124, 4.51] };
        let v2: Vector<Vector<u32>> = Vector { v: vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}] };
        assert_eq!(m1.transform_into_Vector(), v1);
        assert_eq!(m2.transform_into_Vector(), v2);
    }

    #[test]
    fn linear_index() {
        let m1 = Matrix {data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9], col: 3, row: 3};
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
        let ml = Matrix { data: vec![1, 2, 3, 4, 5, 6, 7, 8], row: 1, col: 8};
        let mnl = Matrix { data: vec![1, 2, 3, 4, 5, 6, 7, 8], row: 8, col: 1};
        assert_eq!(v.transform_into_matrix(true), ml);
        assert_eq!(v.transform_into_matrix(false), mnl);
    }

}