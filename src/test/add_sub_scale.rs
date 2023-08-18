
#[cfg(test)]
mod tests {
    
    use crate::matrix::definition::*;

    #[test]
    fn self_add() {
        let mut m1 = Matrix {data: vec![1, 2, 3, 4], col: 2, row: 2};
        let mut m2 = Matrix {data: vec![5, 6, 7, 8], col: 2, row: 2};
        let mut v1 = Vector {v: vec![1, 2, 3, 4]};
        let mut v2 = Vector {v: vec![5, 6, 7, 8]};
        let mut mv1 = Matrix {data: vec![Vector{v: vec![1, 2, 3, 4]}, Vector{v: vec![5, 6, 7, 8]}], col: 2, row: 1};
        m1.self_add(&m2);
        assert_eq!(m1, Matrix {data: vec![6, 8, 10, 12], col: 2, row: 2});
        m2.self_add(&m1);
        assert_eq!(m2, Matrix {data: vec![11, 14, 17, 20], col: 2, row: 2});
        v1.self_add(&v2);
        assert_eq!(v1, Vector {v: vec![6, 8, 10, 12]});
        v2.self_add(&v1);
        assert_eq!(v2, Vector {v: vec![11, 14, 17, 20]});
        mv1.self_add(&Matrix { data: vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}], col: 2, row: 1 });
        assert_eq!(mv1, Matrix { data: vec![Vector {v: vec![2, 4, 6, 8]}, Vector {v: vec![10, 12, 14, 16]}], col: 2, row: 1 });
    }

    #[test]
    fn self_sub() {
        let mut m1 = Matrix {data: vec![1, 2, 3, 4], col: 2, row: 2};
        let mut m2 = Matrix {data: vec![5, 6, 7, 8], col: 2, row: 2};
        let mut v1 = Vector {v: vec![1, 2, 3, 4]};
        let mut v2 = Vector {v: vec![5, 6, 7, 8]};
        let mut mv1 = Matrix {data: vec![Vector{v: vec![1, 2, 3, 4]}, Vector{v: vec![5, 6, 7, 8]}], col: 2, row: 1};
        m1.self_sub(&m2);
        assert_eq!(m1, Matrix {data: vec![-4, -4, -4, -4], col: 2, row: 2});
        m2.self_sub(&m1);
        assert_eq!(m2, Matrix {data: vec![9, 10, 11, 12], col: 2, row: 2});
        v1.self_sub(&v2);
        assert_eq!(v1, Vector {v: vec![-4, -4, -4, -4]});
        v2.self_sub(&v1);
        assert_eq!(v2, Vector {v: vec![9, 10, 11, 12]});
        mv1.self_sub(&Matrix { data: vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}], col: 2, row: 1 });
        assert_eq!(mv1, Matrix { data: vec![Vector {v: vec![0, 0, 0, 0]}, Vector {v: vec![0, 0, 0, 0]}], col: 2, row: 1 });
    }

    #[test]
    fn self_scale() {
        let mut m1 = Matrix {data: vec![1, 2, 3, 4], col: 2, row: 2};
        let mut m2 = Matrix {data: vec![5, 6, 7, 8], col: 2, row: 2};
        let mut v1 = Vector {v: vec![1.5, 2.5, 3.5, 4.5]};
        let mut v2 = Vector {v: vec![5.5, 6.5, 7.5, 8.5]};
        let mut mv1 = Matrix {data: vec![Vector{v: vec![1.0, 2.0, 3.0, 4.0]}, Vector{v: vec![5.0, 6.0, 7.0, 8.0]}], col: 2, row: 1};
        m1.self_scale(2);
        assert_eq!(m1, Matrix {data: vec![2, 4, 6, 8], col: 2, row: 2});
        m2.self_scale(-3);
        assert_eq!(m2, Matrix {data: vec![-15, -18, -21, -24], col: 2, row: 2});
        v1.self_scale(2.0);
        assert_eq!(v1, Vector {v: vec![3.0, 5.0, 7.0, 9.0]});
        v2.self_scale(-1.5);
        assert_eq!(v2, Vector {v: vec![-8.25, -9.75, -11.25, -12.75]});
        mv1.self_scale(Vector {v: vec![1.0, 2.0, 3.0, 4.0]});
        assert_eq!(mv1, Matrix { data: vec![Vector {v: vec![1.0, 2.0, 3.0, 4.0]}, Vector {v: vec![5.0, 6.0, 7.0, 8.0]}], col: 2, row: 1 });
    }
}