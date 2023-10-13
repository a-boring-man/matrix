#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::basic_definition::{definition::{matrix}, complex::Complex};

    #[test]
    fn inverse() {
        let m1 = matrix([[2.0, 0.0], [0.0, 2.0]]);
        assert_eq!(matrix([[0.5, 0.], [0., 0.5]]), m1.inverse().unwrap());

        let m2 = matrix::<i32, 3, 3>::identity();
        assert_eq!(matrix::<i32, 3, 3>::identity(), m2.inverse().unwrap());

        let m3 = matrix([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        let expected_result = matrix([[0.649425287, 0.097701149, -0.655172414], [-0.781609195, -0.126436782, 0.965517241], [0.143678161, 0.074712644, -0.206896552]]);
        let result = m3.inverse().unwrap();
        result.iter().zip(expected_result.iter()).for_each(|(a, b)| {a.iter().zip(b.iter()).for_each(|(ea, eb)| assert_approx_eq!(*ea as f64, *eb as f64))});

        let m3 = matrix([[Complex( 1.,  2.), Complex( 2.,  3.)], [Complex( 3.,  4.), Complex( 4.,  -5.)]]);
        let expected_result = matrix([[Complex( 75. / 298.,  -11. / 149.), Complex( 1. / 298.,  -22. / 149.)], [Complex( -1. / 149.,  -61. / 298.), Complex( -2. / 149.,  27. / 298.)]]);
        let result = m3.inverse().unwrap();
        result.iter().zip(expected_result.iter()).for_each(|(a, b)| {a.iter().zip(b.iter()).for_each(|(ea, eb)| assert_approx_eq!((*ea).0 as f64, (*eb).0 as f64, 1e-2))});
        result.iter().zip(expected_result.iter()).for_each(|(a, b)| {a.iter().zip(b.iter()).for_each(|(ea, eb)| assert_approx_eq!((*ea).1 as f64, (*eb).1 as f64, 1e-2))});


        let m1 = matrix([[2.0, 0.0], [0., 2.]]);
        assert_eq!(matrix([[0.5, 0.], [0., 0.5]]), m1.inverse().unwrap());
    }
}