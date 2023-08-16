
#[cfg(test)]
mod tests {
    
    use crate::matrix::definition::{Matrix, Vector};

    #[test]
    fn it_works() {
        let test_m: Matrix<u32> = Matrix {data: vec![1, 2, 3, 4], row: 2, col: 2};
        println!("{}", test_m);
    }
}