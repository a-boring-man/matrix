#[cfg(test)]
mod tests {
    use crate::matrix::definition::*;

    #[test]
    fn linear_combination() {
        let e1 = Vector::from(vec![1.0, 0.0, 0.0]);
        let e2 = Vector::from(vec![0.0, 1.0, 0.0]);
        let e3 = Vector::from(vec![0.0, 0.0, 1.0]);

        let v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::from(vec![0.0, 10.0, -100.0]);

        assert_eq!(Vector::from(vec![10.0, -2.0, 0.5]), Vector::linear_combination(&[e1, e2, e3], &[10.0, -2.0, 0.5]));
        assert_eq!(Vector::from(vec![10.0, 0.0, 230.0]), Vector::linear_combination(&[v1, v2], &[10.0, -2.0]));
    }
}