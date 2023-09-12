
#[cfg(test)]
mod tests {
    use crate::matrix::basic_definition::{definition::{Matrix, Vector, matrix}, complex::Complex};


    #[test]
    fn self_add() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut m3 = Matrix::from((vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}], 2, 2));
        let mut m4 = Matrix::from((vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}], 2, 2));
        let mut v1 = Vector {v: vec![1, 2, 3, 4]};
        let mut v2 = Vector {v: vec![5, 6, 7, 8]};
        let mut v3 = Vector {v: vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}]};
        let mut v4 = Vector {v: vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}]};
        m1.self_add(&m2);
        assert_eq!(m1, Matrix::from((vec![6, 8, 10, 12], 2, 2)));
        m2.self_add(&m1);
        assert_eq!(m2, Matrix::from((vec![11, 14, 17, 20], 2, 2)));
        m3.self_add(&m4);
        assert_eq!(m3, Matrix::from((vec![Complex {re: 6., im: 8.}, Complex {re: 8., im: 10.}, Complex {re: 10., im: 12.}, Complex {re: 12., im: 14.}], 2, 2)));
        m4.self_add(&m3);
        assert_eq!(m4, Matrix::from((vec![Complex {re: 11., im: 14.}, Complex {re: 14., im: 17.}, Complex {re: 17., im: 20.}, Complex {re: 20., im: 23.}], 2, 2)));
        v1.self_add(&v2);
        assert_eq!(v1, Vector {v: vec![6, 8, 10, 12]});
        v2.self_add(&v1);
        assert_eq!(v2, Vector {v: vec![11, 14, 17, 20]});
        v3.self_add(&v4);
        assert_eq!(v3, Vector {v: vec![Complex {re: 6., im: 8.}, Complex {re: 8., im: 10.}, Complex {re: 10., im: 12.}, Complex {re: 12., im: 14.}]});
        v4.self_add(&v3);
        assert_eq!(v4, Vector {v: vec![Complex {re: 11., im: 14.}, Complex {re: 14., im: 17.}, Complex {re: 17., im: 20.}, Complex {re: 20., im: 23.}]});

        let m1 = matrix([[1, 2], [3, 4]]);
        

    }

    #[test]
    fn self_sub() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut m3 = Matrix::from((vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}], 2, 2));
        let mut m4 = Matrix::from((vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}], 2, 2));
        let mut v1 = Vector {v: vec![1, 2, 3, 4]};
        let mut v2 = Vector {v: vec![5, 6, 7, 8]};
        let mut v3 = Vector {v: vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}]};
        let mut v4 = Vector {v: vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}]};
        m1.self_sub(&m2);
        assert_eq!(m1, Matrix::from((vec![-4, -4, -4, -4], 2, 2)));
        m2.self_sub(&m1);
        assert_eq!(m2, Matrix::from((vec![9, 10, 11, 12], 2, 2)));
        m3.self_sub(&m4);
        assert_eq!(m3, Matrix::from((vec![Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}], 2, 2)));
        m4.self_sub(&m3);
        assert_eq!(m4, Matrix::from((vec![Complex {re: 9., im: 10.}, Complex {re: 10., im: 11.}, Complex {re: 11., im: 12.}, Complex {re: 12., im: 13.}], 2, 2)));
        v1.self_sub(&v2);
        assert_eq!(v1, Vector {v: vec![-4, -4, -4, -4]});
        v2.self_sub(&v1);
        assert_eq!(v2, Vector {v: vec![9, 10, 11, 12]});
        v3.self_sub(&v4);
        assert_eq!(v3, Vector {v: vec![Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}]});
        v4.self_sub(&v3);
        assert_eq!(v4, Vector {v: vec![Complex {re: 9., im: 10.}, Complex {re: 10., im: 11.}, Complex {re: 11., im: 12.}, Complex {re: 12., im: 13.}]});
    }

    #[test]
    fn self_scale() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut m3 = Matrix::from((vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}], 2, 2));
        let mut m4 = Matrix::from((vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}], 2, 2));
        let mut v1 = Vector {v: vec![1.5, 2.5, 3.5, 4.5]};
        let mut v2 = Vector {v: vec![5.5, 6.5, 7.5, 8.5]};
        let mut v3 = Vector {v: vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}]};
        let mut v4 = Vector {v: vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}]};
        m1.self_scale(2);
        assert_eq!(m1, Matrix::from((vec![2, 4, 6, 8], 2, 2)));
        m2.self_scale(-3);
        assert_eq!(m2, Matrix::from((vec![-15, -18, -21, -24], 2, 2)));
        m3.self_scale(Complex { re: 2., im: 0. });
        assert_eq!(m3, Matrix::from((vec![Complex {re: 2., im: 4.}, Complex {re: 4., im: 6.}, Complex {re: 6., im: 8.}, Complex {re: 8., im: 10.}], 2, 2)));
        m4.self_scale(Complex { re: -3., im: 2. });
        assert_eq!(m4, Matrix::from((vec![Complex {re: -27., im: -8.}, Complex {re: -32., im: -9.}, Complex {re: -37., im: -10.}, Complex {re: -42., im: -11.}], 2, 2)));
        v1.self_scale(2.0);
        assert_eq!(v1, Vector {v: vec![3.0, 5.0, 7.0, 9.0]});
        v2.self_scale(-1.5);
        assert_eq!(v2, Vector {v: vec![-8.25, -9.75, -11.25, -12.75]});
        v3.self_scale(Complex { re: 1., im: 4. });
        assert_eq!(v3, Vector {v: vec![Complex {re: -7., im: 6.}, Complex {re: -10., im: 11.}, Complex {re: -13., im: 16.}, Complex {re: -16., im: 21.}]});
        v4.self_scale(Complex { re: -6., im: 3.5 });
        assert_eq!(v4, Vector {v: vec![Complex {re: -51., im: -18.5}, Complex {re: -60.5, im: -21.}, Complex {re: -70., im: -23.5}, Complex {re: -79.5, im: -26.}]});
    }

    #[test]
    fn add() {
        let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let v1 = Vector {v: vec![1, 2, 3, 4]};
        let v2 = Vector {v: vec![5, 6, 7, 8]};
        let m3 = Matrix::from((vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}], 2, 2));
        let m4 = Matrix::from((vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}], 2, 2));
        let v3 = Vector {v: vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}]};
        let v4 = Vector {v: vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}]};
        assert_eq!(m1.add(&m2), Matrix::from((vec![6, 8, 10, 12], 2, 2)));
        assert_eq!(m2.add(&m1), Matrix::from((vec![6, 8, 10, 12], 2, 2)));
        assert_eq!(v1.add(&v2), Vector {v: vec![6, 8, 10, 12]});
        assert_eq!(v2.add(&v1), Vector {v: vec![6, 8, 10, 12]});
        assert_eq!(m3.add(&m4), Matrix::from((vec![Complex {re: 6., im: 8.}, Complex {re: 8., im: 10.}, Complex {re: 10., im: 12.}, Complex {re: 12., im: 14.}], 2, 2)));
        assert_eq!(m4.add(&m3), Matrix::from((vec![Complex {re: 6., im: 8.}, Complex {re: 8., im: 10.}, Complex {re: 10., im: 12.}, Complex {re: 12., im: 14.}], 2, 2)));
        assert_eq!(v3.add(&v4), Vector {v: vec![Complex {re: 6., im: 8.}, Complex {re: 8., im: 10.}, Complex {re: 10., im: 12.}, Complex {re: 12., im: 14.}]});
        assert_eq!(v4.add(&v3), Vector {v: vec![Complex {re: 6., im: 8.}, Complex {re: 8., im: 10.}, Complex {re: 10., im: 12.}, Complex {re: 12., im: 14.}]});
    }

    #[test]
    fn sub() {
        let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let v1 = Vector {v: vec![1, 2, 3, 4]};
        let v2 = Vector {v: vec![5, 6, 7, 8]};
        let m3 = Matrix::from((vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}], 2, 2));
        let m4 = Matrix::from((vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}], 2, 2));
        let v3 = Vector {v: vec![Complex {re: 1., im :2.}, Complex {re: 2., im: 3.}, Complex {re: 3., im: 4.}, Complex {re: 4., im: 5.}]};
        let v4 = Vector {v: vec![Complex {re: 5., im :6.}, Complex {re: 6., im: 7.}, Complex {re: 7., im: 8.}, Complex {re: 8., im: 9.}]};
        assert_eq!(m1.sub(&m2), Matrix::from((vec![-4, -4, -4, -4], 2, 2)));
        assert_eq!(m2.sub(&m1), Matrix::from((vec![4, 4, 4, 4], 2, 2)));
        assert_eq!(v1.sub(&v2), Vector {v: vec![-4, -4, -4, -4]});
        assert_eq!(v2.sub(&v1), Vector {v: vec![4, 4, 4, 4]});
        assert_eq!(m3.sub(&m4), Matrix::from((vec![Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}], 2, 2)));
        assert_eq!(m4.sub(&m3), Matrix::from((vec![Complex {re: 4., im: 4.}, Complex {re: 4., im: 4.}, Complex {re: 4., im: 4.}, Complex {re: 4., im: 4.}], 2, 2)));
        assert_eq!(v3.sub(&v4), Vector {v: vec![Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}, Complex {re: -4., im: -4.}]});
        assert_eq!(v4.sub(&v3), Vector {v: vec![Complex {re: 4., im: 4.}, Complex {re: 4., im: 4.}, Complex {re: 4., im: 4.}, Complex {re: 4., im: 4.}]});
    }

    #[test]
    fn scale() {
        let m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let v1 = Vector {v: vec![1.5, 2.5, 3.5, 4.5]};
        let v2 = Vector {v: vec![5.5, 6.5, 7.5, 8.5]};
        assert_eq!(m1.scale(2), Matrix::from((vec![2, 4, 6, 8], 2, 2)));
        assert_eq!(m2.scale(-3), Matrix::from((vec![-15, -18, -21, -24], 2, 2)));
        assert_eq!(v1.scale(&2.0), Vector {v: vec![3.0, 5.0, 7.0, 9.0]});
        assert_eq!(v2.scale(&-1.5), Vector {v: vec![-8.25, -9.75, -11.25, -12.75]});
    }

}