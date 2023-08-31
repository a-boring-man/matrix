
#[cfg(test)]
mod tests {
    use crate::matrix::definition::{Matrix, Vector};

    #[test]
    fn self_add() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut v1 = Vector {v: vec![1, 2, 3, 4]};
        let mut v2 = Vector {v: vec![5, 6, 7, 8]};
        let mut mv1 = Matrix::from((vec![Vector{v: vec![1, 2, 3, 4]}, Vector{v: vec![5, 6, 7, 8]}], 2, 1));
        m1.self_add(&m2);
        assert_eq!(m1, Matrix::from((vec![6, 8, 10, 12], 2, 2)));
        m2.self_add(&m1);
        assert_eq!(m2, Matrix::from((vec![11, 14, 17, 20], 2, 2)));
        v1.self_add(&v2);
        assert_eq!(v1, Vector {v: vec![6, 8, 10, 12]});
        v2.self_add(&v1);
        assert_eq!(v2, Vector {v: vec![11, 14, 17, 20]});
        mv1.self_add(&Matrix::from((vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}], 2, 1 )));
        assert_eq!(mv1, Matrix::from((vec![Vector {v: vec![2, 4, 6, 8]}, Vector {v: vec![10, 12, 14, 16]}], 2, 1 )));
    }

    #[test]
    fn self_sub() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut v1 = Vector {v: vec![1, 2, 3, 4]};
        let mut v2 = Vector {v: vec![5, 6, 7, 8]};
        let mut mv1 = Matrix::from((vec![Vector{v: vec![1, 2, 3, 4]}, Vector{v: vec![5, 6, 7, 8]}], 2, 1));
        m1.self_sub(&m2);
        assert_eq!(m1, Matrix::from((vec![-4, -4, -4, -4], 2, 2)));
        m2.self_sub(&m1);
        assert_eq!(m2, Matrix::from((vec![9, 10, 11, 12], 2, 2)));
        v1.self_sub(&v2);
        assert_eq!(v1, Vector {v: vec![-4, -4, -4, -4]});
        v2.self_sub(&v1);
        assert_eq!(v2, Vector {v: vec![9, 10, 11, 12]});
        mv1.self_sub(&Matrix::from((vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}], 2, 1 )));
        assert_eq!(mv1, Matrix::from((vec![Vector {v: vec![0, 0, 0, 0]}, Vector {v: vec![0, 0, 0, 0]}], 2, 1 )));
    }

    #[test]
    fn self_scale() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut v1 = Vector {v: vec![1.5, 2.5, 3.5, 4.5]};
        let mut v2 = Vector {v: vec![5.5, 6.5, 7.5, 8.5]};
        let mut mv1 = Matrix::from((vec![Vector{v: vec![1.0, 2.0, 3.0, 4.0]}, Vector{v: vec![5.0, 6.0, 7.0, 8.0]}], 2, 1));
        m1.self_scale(2);
        assert_eq!(m1, Matrix::from((vec![2, 4, 6, 8], 2, 2)));
        m2.self_scale(-3);
        assert_eq!(m2, Matrix::from((vec![-15, -18, -21, -24], 2, 2)));
        v1.self_scale(2.0);
        assert_eq!(v1, Vector {v: vec![3.0, 5.0, 7.0, 9.0]});
        v2.self_scale(-1.5);
        assert_eq!(v2, Vector {v: vec![-8.25, -9.75, -11.25, -12.75]});
        mv1.self_scale(Vector {v: vec![1.0, 2.0, 3.0, 4.0]});
        assert_eq!(mv1, Matrix::from((vec![Vector {v: vec![1.0, 2.0, 3.0, 4.0]}, Vector {v: vec![5.0, 6.0, 7.0, 8.0]}], 2, 1 )));
    }

    #[test]
    fn add() {
        let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let v1 = Vector {v: vec![1, 2, 3, 4]};
        let v2 = Vector {v: vec![5, 6, 7, 8]};
        let mv1 = Matrix::from((vec![Vector{v: vec![1, 2, 3, 4]}, Vector{v: vec![5, 6, 7, 8]}], 2, 1));
        let mv2 = Matrix::from(( vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}], 2, 1 ));
        assert_eq!(m1.add(&m2), Matrix::from((vec![6, 8, 10, 12], 2, 2)));
        assert_eq!(m2.add(&m1), Matrix::from((vec![6, 8, 10, 12], 2, 2)));
        assert_eq!(v1.add(&v2), Vector {v: vec![6, 8, 10, 12]});
        assert_eq!(v2.add(&v1), Vector {v: vec![6, 8, 10, 12]});
        assert_eq!(mv1.add(&mv2), Matrix::from((vec![Vector {v: vec![2, 4, 6, 8]}, Vector {v: vec![10, 12, 14, 16]}], 2, 1 )));
    }

    #[test]
    fn sub() {
        let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let v1 = Vector {v: vec![1, 2, 3, 4]};
        let v2 = Vector {v: vec![5, 6, 7, 8]};
        let mv1 = Matrix::from((vec![Vector{v: vec![1, 2, 3, 4]}, Vector{v: vec![5, 6, 7, 8]}], 2, 1));
        let mv2 = Matrix::from((vec![Vector {v: vec![1, 2, 3, 4]}, Vector {v: vec![5, 6, 7, 8]}], 2, 1));
        assert_eq!(m1.sub(&m2), Matrix::from((vec![-4, -4, -4, -4], 2, 2)));
        assert_eq!(m2.sub(&m1), Matrix::from((vec![4, 4, 4, 4], 2, 2)));
        assert_eq!(v1.sub(&v2), Vector {v: vec![-4, -4, -4, -4]});
        assert_eq!(v2.sub(&v1), Vector {v: vec![4, 4, 4, 4]});
        assert_eq!(mv1.sub(&mv2), Matrix::from((vec![Vector {v: vec![0, 0, 0, 0]}, Vector {v: vec![0, 0, 0, 0]}], 2, 1 )));
    }

    #[test]
    fn scale() {
        let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let v1 = Vector {v: vec![1.5, 2.5, 3.5, 4.5]};
        let v2 = Vector {v: vec![5.5, 6.5, 7.5, 8.5]};
        let mv1 = Matrix::from((vec![Vector{v: vec![1.0, 2.0, 3.0, 4.0]}, Vector{v: vec![5.0, 6.0, 7.0, 8.0]}], 2, 1));
        assert_eq!(m1.scale(2), Matrix::from((vec![2, 4, 6, 8], 2, 2)));
        assert_eq!(m2.scale(-3), Matrix::from((vec![-15, -18, -21, -24], 2, 2)));
        assert_eq!(v1.scale(&2.0), Vector {v: vec![3.0, 5.0, 7.0, 9.0]});
        assert_eq!(v2.scale(&-1.5), Vector {v: vec![-8.25, -9.75, -11.25, -12.75]});
        assert_eq!(mv1.scale(Vector {v: vec![1.0, 2.0, 3.0, 4.0]}), Matrix::from((vec![Vector {v: vec![1.0, 2.0, 3.0, 4.0]}, Vector {v: vec![5.0, 6.0, 7.0, 8.0]}], 2, 1 )));
    }

}