#[cfg(test)]
mod tests {
    use crate::matrix::basic_definition::{definition::{matrix, vector}, complex::Complex};

    #[test]
    fn matrix_printing_test() {
        println!("{}\n", matrix([[1, 2], [3, 4]]));
        println!("{}\n", matrix([[1., 2., 3.], [4., -5., 6.], [7., 8., 9.]]));
        println!("{}\n", matrix([[Complex(1., 2.), Complex(3., 4.)]]));
    }

    #[test]
    fn vector_printing_test() {
        println!("{}\n", vector([1, 2, 3, 4]));
        println!("{}\n", vector([1., 2., 3., -4., 5., 6., 7., 8., 9.]));
        println!("{}\n", vector([Complex(4., 5.), Complex(3., 4.2)]));
    }

    #[test]
    fn identity() {
        assert_eq!(matrix::<i32, 2, 2>::identity(), matrix([[1, 0], [0, 1]]));
        assert_eq!(matrix::<f32, 3, 3>::identity(), matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]));
        assert_eq!(matrix::<Complex, 2, 2>::identity(), matrix([[Complex(1., 0.), Complex(0., 0.)], [Complex(0., 0.), Complex(1., 0.)]]));
    }

    #[test]
    fn new() {
        assert_eq!(matrix::<i32, 3, 2>::new(), matrix([[0, 0], [0, 0], [0, 0]]));
        assert_eq!(matrix::<Complex, 1, 2>::new(), matrix([[Complex(0., 0.), Complex(0., 0.)]]));

        assert_eq!(vector::<i32, 2>::new(), vector([0, 0]));
        assert_eq!(vector::<Complex, 2>::new(), vector([Complex(0., 0.), Complex(0., 0.)]));
    }

    #[test]
    fn isolate_column_vector() {
        assert_eq!(matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]).isolate_column_vector(2), vector([3, 6, 9]));
    }

    #[test]
    fn from() {
        let salut = [[1, 2], [3, 4]];
        assert_eq!(matrix([[1, 2], [3, 4]]), matrix::from(salut));
    }

}