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
    }

    #[test]
    fn vector_printing_test() {
        let test_m: Vector<u32> = Vector { v: vec![1, 2, 3, 4]};
        println!("{}", test_m);
    }
}