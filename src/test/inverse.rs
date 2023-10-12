#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{Matrix, matrix}, complex::Complex};

    #[test]
    fn inverse() {
        let m1 = Matrix::from((vec![2.0, 0.0, 0.0, 2.0], 2, 2));
        assert_eq!(Matrix::from((vec![0.5, 0., 0., 0.5], 2, 2)), m1.inverse().unwrap());

        let m2 = Matrix::<i32>::identity(3);
        assert_eq!(Matrix::<i32>::identity(3), m2.inverse().unwrap());

        let m3 = Matrix::from((vec![8., 5., -2., 4., 7., 20., 7., 6., 1.], 3, 3));
        let expected_result = Matrix::from((vec![0.649425287, 0.097701149, -0.655172414, -0.781609195, -0.126436782, 0.965517241, 0.143678161, 0.074712644, -0.206896552], 3, 3));
        let result = m3.inverse().unwrap();
        result.iter().zip(expected_result.iter()).for_each(|(a, b)| assert_approx_eq!(*a, *b));

        let m3 = Matrix::from((vec![Complex( 1.,  2.), Complex( 2.,  3.), Complex( 3.,  4.), Complex( 4.,  -5.)], 2, 2));
        let expected_result = Matrix::from((vec![Complex( 75. / 298.,  -11. / 149.), Complex( 1. / 298.,  -22. / 149.), Complex( -1. / 149.,  -61. / 298.), Complex( -2. / 149.,  27. / 298.)], 2, 2));
        let result = m3.inverse().unwrap();
        result.iter().zip(expected_result.iter()).for_each(|(a, b)| assert_approx_eq!((*a).0 as f32, (*b).0 as f32, 1e-2));
        result.iter().zip(expected_result.iter()).for_each(|(a, b)| assert_approx_eq!((*a).1 as f32, (*b).1 as f32, 1e-2));

        let m1 = matrix([[2.0, 0.0], [0., 2.]]);
        assert_eq!(matrix([[0.5, 0.], [0., 0.5]]), m1.inverse().unwrap());
    }
}