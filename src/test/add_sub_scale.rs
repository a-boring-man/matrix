
#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{Matrix, Vector, matrix, vector}, complex::Complex};


    #[test]
    fn self_add() {

        let mut m1 = matrix([[1, 2], [3, 4]]);
        m1 += matrix([[5, 6], [7, 8]]);
        assert_eq!(m1, matrix([[6, 8], [10, 12]]));
        
        let mut v1 = vector([1, 2]);
        v1 += vector([3, 4]);
        assert_eq!(v1, vector([4, 6]));

        let mut m1 = matrix([[1, 2], [3, 4]]);
        let m2 = matrix([[5, 6], [7, 8]]);
        m1 += m2;
        assert!(m1 == matrix([[6, 8], [10, 12]]));

        let mut ma = matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let mb = matrix([[1, 3, 5], [7, 9, 11], [13, 15, 17]]);
        ma += mb;
        assert!(ma == matrix([[3, -1, 11], [15, 19, 23], [27, 31, 35]]));

        let mut a = matrix([[2., 3.], [1., 4.]]);
        let b = matrix([[5., 1.], [2., 6.]]);
        a += b;
        assert!(a == matrix([[7., 4.], [3., 10.]]));

        let mut x = matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let y = matrix([[Complex(2., 0.), Complex(0., 3.)], [Complex(3., 4.), Complex(1., 2.)]]);
        x += y;
        assert!(x == matrix([[Complex(2., 1.), Complex(1., 3.)], [Complex(3., 4.), Complex(2., 3.)]]));

        let mut p = vector([1, 2, 3]);
        let q = vector([6, 5, 4]);
        p += q;
        assert!(p == vector([7, 7, 7]));

        let mut heu = vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let salut = vector([Complex(2., 1.), Complex(3., 1.)]);
        heu += salut;
        assert_eq!(heu, vector([Complex(2., 3.), Complex(-1., 4.4)]));
    }

    #[test]
    fn self_sub() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut m3 = Matrix::from((vec![Complex( 1., 2.), Complex( 2.,  3.), Complex( 3.,  4.), Complex( 4.,  5.)], 2, 2));
        let mut m4 = Matrix::from((vec![Complex( 5., 6.), Complex( 6.,  7.), Complex( 7.,  8.), Complex( 8.,  9.)], 2, 2));
        let mut v1 = Vector {v: vec![1, 2, 3, 4]};
        let mut v2 = Vector {v: vec![5, 6, 7, 8]};
        let mut v3 = Vector {v: vec![Complex( 1., 2.), Complex( 2.,  3.), Complex( 3.,  4.), Complex( 4.,  5.)]};
        let mut v4 = Vector {v: vec![Complex( 5., 6.), Complex( 6.,  7.), Complex( 7.,  8.), Complex( 8.,  9.)]};
        m1.self_sub(&m2);
        assert_eq!(m1, Matrix::from((vec![-4, -4, -4, -4], 2, 2)));
        m2.self_sub(&m1);
        assert_eq!(m2, Matrix::from((vec![9, 10, 11, 12], 2, 2)));
        m3.self_sub(&m4);
        assert_eq!(m3, Matrix::from((vec![Complex( -4.,  -4.), Complex( -4.,  -4.), Complex( -4.,  -4.), Complex( -4.,  -4.)], 2, 2)));
        m4.self_sub(&m3);
        assert_eq!(m4, Matrix::from((vec![Complex( 9.,  10.), Complex( 10.,  11.), Complex( 11.,  12.), Complex( 12.,  13.)], 2, 2)));
        v1.self_sub(&v2);
        assert_eq!(v1, Vector {v: vec![-4, -4, -4, -4]});
        v2.self_sub(&v1);
        assert_eq!(v2, Vector {v: vec![9, 10, 11, 12]});
        v3.self_sub(&v4);
        assert_eq!(v3, Vector {v: vec![Complex( -4.,  -4.), Complex( -4.,  -4.), Complex( -4.,  -4.), Complex( -4.,  -4.)]});
        v4.self_sub(&v3);
        assert_eq!(v4, Vector {v: vec![Complex( 9.,  10.), Complex( 10.,  11.), Complex( 11.,  12.), Complex( 12.,  13.)]});

        let a = matrix([[2, 3], [1, 4]]);
        let b = matrix([[5, 1], [2, 6]]);
        assert!(a - b == matrix([[-3, 2], [-1, -2]]));

        let x = matrix([[0, 1], [1, 0]]);
        let y = matrix([[1, 0], [0, 1]]);
        assert!(x - y == matrix([[-1, 1], [1, -1]]));

        let p = matrix([[1, 2, 3], [4, 5, 6]]);
        let q = matrix([[6, 5, 4], [3, 2, 1]]);
        assert!(p - q == matrix([[-5, -3, -1], [1, 3, 5]]));

        let m = matrix([[8, 7, 6], [5, 4, 3], [2, 1, 0]]);
        let n = matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert!(m - n == matrix([[7, 5, 3], [1, -1, -3], [-5, -7, -9]]));

        let r = matrix([[10, 20], [30, 40]]);
        let s = matrix([[5, 10], [15, 20]]);
        assert!(r - s == matrix([[5, 10], [15, 20]]));

    }

    #[test]
    fn self_scale() {
        let mut m1 = Matrix::from((vec![1, 2, 3, 4], 2, 2));
        let mut m2 = Matrix::from((vec![5, 6, 7, 8], 2, 2));
        let mut m3 = Matrix::from((vec![Complex( 1., 2.), Complex( 2.,  3.), Complex( 3.,  4.), Complex( 4.,  5.)], 2, 2));
        let mut m4 = Matrix::from((vec![Complex( 5., 6.), Complex( 6.,  7.), Complex( 7.,  8.), Complex( 8.,  9.)], 2, 2));
        let mut v1 = Vector {v: vec![1.5, 2.5, 3.5, 4.5]};
        let mut v2 = Vector {v: vec![5.5, 6.5, 7.5, 8.5]};
        let mut v3 = Vector {v: vec![Complex( 1., 2.), Complex( 2.,  3.), Complex( 3.,  4.), Complex( 4.,  5.)]};
        let mut v4 = Vector {v: vec![Complex( 5., 6.), Complex( 6.,  7.), Complex( 7.,  8.), Complex( 8.,  9.)]};
        m1.self_scale(2);
        assert_eq!(m1, Matrix::from((vec![2, 4, 6, 8], 2, 2)));
        m2.self_scale(-3);
        assert_eq!(m2, Matrix::from((vec![-15, -18, -21, -24], 2, 2)));
        m3.self_scale(Complex( 2.,  0.));
        assert_eq!(m3, Matrix::from((vec![Complex( 2.,  4.), Complex( 4.,  6.), Complex( 6.,  8.), Complex( 8.,  10.)], 2, 2)));
        m4.self_scale(Complex( -3.,  2.));
        assert_eq!(m4, Matrix::from((vec![Complex( -27.,  -8.), Complex( -32.,  -9.), Complex( -37.,  -10.), Complex( -42.,  -11.)], 2, 2)));
        v1.self_scale(2.0);
        assert_eq!(v1, Vector {v: vec![3.0, 5.0, 7.0, 9.0]});
        v2.self_scale(-1.5);
        assert_eq!(v2, Vector {v: vec![-8.25, -9.75, -11.25, -12.75]});
        v3.self_scale(Complex( 1.,  4.));
        assert_eq!(v3, Vector {v: vec![Complex( -7.,  6.), Complex( -10.,  11.), Complex( -13.,  16.), Complex( -16.,  21.)]});
        v4.self_scale(Complex( -6.,  3.5 ));
        assert_eq!(v4, Vector {v: vec![Complex( -51.,  -18.5), Complex( -60.5,  -21.), Complex( -70.,  -23.5), Complex( -79.5,  -26.)]});

        let a = matrix([[2, 3], [1, 4]]);
        let s = 2;
        assert_eq!(a * s, matrix([[4, 6], [2, 8]]));

        let b = matrix([[5, 1], [2, 6]]);
        let s = 3;
        assert!(b * s == matrix([[15, 3], [6, 18]]));

        let x = matrix([[0, 1], [1, 0]]);
        let s = -1;
        assert!(x * s == matrix([[0, -1], [-1, 0]]));

        let y = matrix([[1, 2, 3], [4, 5, 6]]);
        let s = 0;
        assert!(y * s == matrix([[0, 0, 0], [0, 0, 0]]));

        let z = matrix([[10., 20., 30.], [40., 50., 60.], [70., 80., 90.]]);
        let s = 0.5;
        assert!(z * s == matrix([[5.0, 10.0, 15.0], [20.0, 25.0, 30.0], [35.0, 40.0, 45.0]]));

        let a = vector([3, 4]);
        assert_eq!(vector([6, 8]), a * 2);
    }

    #[test]
    fn add() {
        let m1 = matrix([[1, 2], [3, 4]]);
        let m2 = matrix([[5, 6], [7, 8]]);
        assert!(m1 + m2 == matrix([[6, 8], [10, 12]]));

        let ma = matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let mb = matrix([[1, 3, 5], [7, 9, 11], [13, 15, 17]]);
        assert!(ma + mb == matrix([[3, -1, 11], [15, 19, 23], [27, 31, 35]]));

        let a = matrix([[2., 3.], [1., 4.]]);
        let b = matrix([[5., 1.], [2., 6.]]);
        assert!(a + b == matrix([[7., 4.], [3., 10.]]));

        let x = matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let y = matrix([[Complex(2., 0.), Complex(0., 3.)], [Complex(3., 4.), Complex(1., 2.)]]);
        assert!(x + y == matrix([[Complex(2., 1.), Complex(1., 3.)], [Complex(3., 4.), Complex(2., 3.)]]));

        let p = vector([1, 2, 3]);
        let q = vector([6, 5, 4]);
        assert!(p + q == vector([7, 7, 7]));

        let heu = vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let salut = vector([Complex(2., 1.), Complex(3., 1.)]);
        assert_eq!(heu + salut, vector([Complex(2., 3.), Complex(-1., 4.4)]));
    }

    #[test]
    fn sub() {
        let m1 = matrix([[1, 2], [3, 4]]);
        let m2 = matrix([[5, 6], [7, 8]]);
        assert!(m1 - m2 == matrix([[-4, -4], [-4, -4]]));

        let ma = matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let mb = matrix([[1, 3, 5], [7, 9, 11], [13, 15, 17]]);
        assert!(ma - mb == matrix([[1, -7, 1], [1, 1, 1], [1, 1, 1]]));

        let a = matrix([[2., 3.], [1., 4.]]);
        let b = matrix([[5., 1.], [2., 6.]]);
        assert!(a - b == matrix([[-3., 2.], [-1., -2.]]));

        let x = matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let y = matrix([[Complex(2., 0.), Complex(0., 3.)], [Complex(3., 4.), Complex(1., 2.)]]);
        assert!(x - y == matrix([[Complex(-2., 1.), Complex(1., -3.)], [Complex(-3., -4.), Complex(0., -1.)]]));
        
        let p = vector([1, 2, 3]);
        let q = vector([6, 5, 4]);
        assert!(p - q == vector([-5, -3, -1]));
        
        let heu = vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let salut = vector([Complex(2., 1.), Complex(3., 1.)]);
        assert_eq!(heu - salut, vector([Complex(-2., 1.), Complex(-7., 2.4)]));
        
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
        
        let m1 = matrix([[1, 2], [3, 4]]);
        let scalar = 2;
        assert!(m1 * scalar == matrix([[2, 4], [6, 8]]));
        
        let ma = matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let scalar = -1;
        assert!(ma * scalar == matrix([[-2, 4, -6], [-8, -10, -12], [-14, -16, -18]]));
        
        let a = matrix([[2., 3.], [1., 4.]]);
        let scalar = 0.5;
        assert!(a * scalar == matrix([[1.0, 1.5], [0.5, 2.0]]));
        
        let x = matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let scalar = Complex(2., 0.);
        assert!(x * scalar == matrix([[Complex(0., 2.), Complex(2., 0.)], [Complex(0., 0.), Complex(2., 2.)]]));
        
        let p = vector([1, 2, 3]);
        let scalar = 3;
        assert!(p * scalar == vector([3, 6, 9]));
        
        let heu = vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let scalar = Complex(2., 1.);
        let result = heu * scalar;
        for (elm, res) in result.iter().zip(result.iter()) {
            assert_approx_eq!(elm.0, res.0);
            assert_approx_eq!(elm.1, res.1);
        }
    }
                    
}