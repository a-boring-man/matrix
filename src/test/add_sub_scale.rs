
#[cfg(test)]
mod tests {
    
    use crate::matrix::definition::*;

    #[test]
    fn self_add() {
        let mut m1 = Matrix {data: vec![1, 2, 3, 4], col: 2, row: 2};
        let mut m2 = Matrix {data: vec![5, 6, 7, 8], col: 2, row: 2};
        m1.self_add(&m2);
        assert_eq!(m1, Matrix {data: vec![6, 8, 10, 12], col: 2, row: 2});
        m2.self_add(&m1);
        assert_eq!(m2, Matrix {data: vec![11, 14, 17, 20], col: 2, row: 2});
    }

    #[test]
    fn self_sub() {
        let mut m1 = Matrix {data: vec![1, 2, 3, 4], col: 2, row: 2};
        let mut m2 = Matrix {data: vec![5, 6, 7, 8], col: 2, row: 2};
        m1.self_sub(&m2);
        assert_eq!(m1, Matrix {data: vec![-4, -4, -4, -4], col: 2, row: 2});
        m2.self_sub(&m1);
        assert_eq!(m2, Matrix {data: vec![9, 10, 11, 12], col: 2, row: 2});
    }

}