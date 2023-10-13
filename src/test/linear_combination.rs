#[cfg(test)]
mod tests {
    use crate::matrix::{basic_definition::{definition::{vector}, complex::Complex}, vector_impl::linear_combination::_linear_combination};

    #[test]
    fn linear_combination() {
        let e1 = vector([1., 0., 0.]);
        let e2 = vector([0., 1., 0.]);
        let e3 = vector([0., 0., 1.]);
        
        let v1 = vector([1.0, 2.0, 3.0]);
        let v2 = vector([0.0, 10.0, -100.0]);

        let rc1 = vector([Complex( 0.5,  8.5), Complex( 2.5,  13.5), Complex( 4.5,  18.5)]);
        let c1 = vector([Complex( 1.,  2.), Complex( 2.,  3.), Complex( 3.,  4.)]);

        assert_eq!(vector([10., -2., 0.5]), _linear_combination([e1, e2, e3], [10., -2., 0.5]));
        assert_eq!(vector([10., 0., 230.]), _linear_combination([v1, v2], [10., -2.]));
        assert_eq!(rc1, _linear_combination([c1], [Complex(3.5, 1.5)]));
    }
}