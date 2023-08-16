
#[cfg(test)]
mod tests {
    
    use crate::matrix::definition::{Matrix, Vector};

    #[test]
    fn matrix_printing_test() {
        let test_m: Matrix<u32> = Matrix {data: vec![1, 2, 3, 4], row: 2, col: 2};
        println!("{}", test_m);
    }

    #[test]
    fn vector_printing_test() {
        let test_m: Vector<u32> = Vector { v: vec![1, 2, 3, 4]};
        println!("{}", test_m);
    }
}