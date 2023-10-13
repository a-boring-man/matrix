
#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{Matrix, Vector}, complex::Complex};


    #[test]
    fn self_add() {

        let mut m1 = Matrix([[1, 2], [3, 4]]);
        m1 += Matrix([[5, 6], [7, 8]]);
        assert_eq!(m1, Matrix([[6, 8], [10, 12]]));
        
        let mut v1 = Vector([1, 2]);
        v1 += Vector([3, 4]);
        assert_eq!(v1, Vector([4, 6]));

        let mut m1 = Matrix([[1, 2], [3, 4]]);
        let m2 = Matrix([[5, 6], [7, 8]]);
        m1 += m2;
        assert!(m1 == Matrix([[6, 8], [10, 12]]));

        let mut ma = Matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let mb = Matrix([[1, 3, 5], [7, 9, 11], [13, 15, 17]]);
        ma += mb;
        assert!(ma == Matrix([[3, -1, 11], [15, 19, 23], [27, 31, 35]]));

        let mut a = Matrix([[2., 3.], [1., 4.]]);
        let b = Matrix([[5., 1.], [2., 6.]]);
        a += b;
        assert!(a == Matrix([[7., 4.], [3., 10.]]));

        let mut x = Matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let y = Matrix([[Complex(2., 0.), Complex(0., 3.)], [Complex(3., 4.), Complex(1., 2.)]]);
        x += y;
        assert!(x == Matrix([[Complex(2., 1.), Complex(1., 3.)], [Complex(3., 4.), Complex(2., 3.)]]));

        let mut p = Vector([1, 2, 3]);
        let q = Vector([6, 5, 4]);
        p += q;
        assert!(p == Vector([7, 7, 7]));

        let mut heu = Vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let salut = Vector([Complex(2., 1.), Complex(3., 1.)]);
        heu += salut;
        assert_eq!(heu, Vector([Complex(2., 3.), Complex(-1., 4.4)]));
    }

    #[test]
    fn add() {
        let m1 = Matrix([[1, 2], [3, 4]]);
        let m2 = Matrix([[5, 6], [7, 8]]);
        assert!(m1 + m2 == Matrix([[6, 8], [10, 12]]));

        let ma = Matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let mb = Matrix([[1, 3, 5], [7, 9, 11], [13, 15, 17]]);
        assert!(ma + mb == Matrix([[3, -1, 11], [15, 19, 23], [27, 31, 35]]));

        let a = Matrix([[2., 3.], [1., 4.]]);
        let b = Matrix([[5., 1.], [2., 6.]]);
        assert!(a + b == Matrix([[7., 4.], [3., 10.]]));

        let x = Matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let y = Matrix([[Complex(2., 0.), Complex(0., 3.)], [Complex(3., 4.), Complex(1., 2.)]]);
        assert!(x + y == Matrix([[Complex(2., 1.), Complex(1., 3.)], [Complex(3., 4.), Complex(2., 3.)]]));

        let p = Vector([1, 2, 3]);
        let q = Vector([6, 5, 4]);
        assert!(p + q == Vector([7, 7, 7]));

        let heu = Vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let salut = Vector([Complex(2., 1.), Complex(3., 1.)]);
        assert_eq!(heu + salut, Vector([Complex(2., 3.), Complex(-1., 4.4)]));
    }

    #[test]
    fn sub() {
        let m1 = Matrix([[1, 2], [3, 4]]);
        let m2 = Matrix([[5, 6], [7, 8]]);
        assert!(m1 - m2 == Matrix([[-4, -4], [-4, -4]]));

        let ma = Matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let mb = Matrix([[1, 3, 5], [7, 9, 11], [13, 15, 17]]);
        assert!(ma - mb == Matrix([[1, -7, 1], [1, 1, 1], [1, 1, 1]]));

        let a = Matrix([[2., 3.], [1., 4.]]);
        let b = Matrix([[5., 1.], [2., 6.]]);
        assert!(a - b == Matrix([[-3., 2.], [-1., -2.]]));

        let x = Matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let y = Matrix([[Complex(2., 0.), Complex(0., 3.)], [Complex(3., 4.), Complex(1., 2.)]]);
        assert!(x - y == Matrix([[Complex(-2., 1.), Complex(1., -3.)], [Complex(-3., -4.), Complex(0., -1.)]]));
        
        let p = Vector([1, 2, 3]);
        let q = Vector([6, 5, 4]);
        assert!(p - q == Vector([-5, -3, -1]));
        
        let heu = Vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let salut = Vector([Complex(2., 1.), Complex(3., 1.)]);
        assert_eq!(heu - salut, Vector([Complex(-2., 1.), Complex(-7., 2.4)]));
        
    }

    #[test]
    fn scale() {
        let m1 = Matrix([[1, 2], [3, 4]]);
        let scalar = 2;
        assert!(m1 * scalar == Matrix([[2, 4], [6, 8]]));
        
        let ma = Matrix([[2, -4, 6], [8, 10, 12], [14, 16, 18]]);
        let scalar = -1;
        assert!(ma * scalar == Matrix([[-2, 4, -6], [-8, -10, -12], [-14, -16, -18]]));
        
        let a = Matrix([[2., 3.], [1., 4.]]);
        let scalar = 0.5;
        assert!(a * scalar == Matrix([[1.0, 1.5], [0.5, 2.0]]));
        
        let x = Matrix([[Complex(0., 1.), Complex(1., 0.)], [Complex(0., 0.), Complex(1., 1.)]]);
        let scalar = Complex(2., 0.);
        assert!(x * scalar == Matrix([[Complex(0., 2.), Complex(2., 0.)], [Complex(0., 0.), Complex(2., 2.)]]));
        
        let p = Vector([1, 2, 3]);
        let scalar = 3;
        assert!(p * scalar == Vector([3, 6, 9]));
        
        let heu = Vector([Complex(0., 2.), Complex(-4., 3.4)]);
        let scalar = Complex(2., 1.);
        let result = heu * scalar;
        for (elm, res) in result.iter().zip(result.iter()) {
            assert_approx_eq!(elm.0, res.0);
            assert_approx_eq!(elm.1, res.1);
        }
    }
                    
}