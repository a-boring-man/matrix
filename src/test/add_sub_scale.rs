
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