#[cfg(test)]
mod tests {
    use crate::matrix::basic_definition::{definition::{Matrix, Vector}, complex::Complex};

    #[test]
    fn matrix_printing_test() {
        println!("{}\n", Matrix([[1, 2], [3, 4]]));
        println!("{}\n", Matrix([[1., 2., 3.], [4., -5., 6.], [7., 8., 9.]]));
        println!("{}\n", Matrix([[Complex(1., 2.), Complex(3., 4.)]]));
    }

    #[test]
    fn vector_printing_test() {
        println!("{}\n", Vector([1, 2, 3, 4]));
        println!("{}\n", Vector([1., 2., 3., -4., 5., 6., 7., 8., 9.]));
        println!("{}\n", Vector([Complex(4., 5.), Complex(3., 4.2)]));
    }

    #[test]
    fn identity() {
        assert_eq!(Matrix::<i32, 2, 2>::identity(), Matrix([[1, 0], [0, 1]]));
        assert_eq!(Matrix::<f32, 3, 3>::identity(), Matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]));
        assert_eq!(Matrix::<Complex, 2, 2>::identity(), Matrix([[Complex(1., 0.), Complex(0., 0.)], [Complex(0., 0.), Complex(1., 0.)]]));
    }

    #[test]
    fn new() {
        assert_eq!(Matrix::<i32, 3, 2>::new(), Matrix([[0, 0], [0, 0], [0, 0]]));
        assert_eq!(Matrix::<Complex, 1, 2>::new(), Matrix([[Complex(0., 0.), Complex(0., 0.)]]));

        assert_eq!(Vector::<i32, 2>::new(), Vector([0, 0]));
        assert_eq!(Vector::<Complex, 2>::new(), Vector([Complex(0., 0.), Complex(0., 0.)]));
    }

    #[test]
    fn isolate_column_vector() {
        assert_eq!(Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]).isolate_column_vector(2), Vector([3, 6, 9]));
    }

    #[test]
    fn from() {
        let salut = [[1, 2], [3, 4]];
        assert_eq!(Matrix([[1, 2], [3, 4]]), Matrix::from(salut));
    }

}