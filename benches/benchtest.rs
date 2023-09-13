use criterion::{Criterion, criterion_group, criterion_main, black_box};
use matrix::matrix::basic_definition::definition::vector;
use ::matrix::matrix::basic_definition::definition::{Matrix, Vector};
use matrix::matrix::vector_impl::linear_combination::linear_combination;

fn benchtest(c: &mut Criterion) {
	let e1 = black_box(vector([1.0, 0.0, 0.0]));
	let e2 = black_box(vector([0.0, 1.0, 0.0]));
	let e3 = black_box(vector([0.0, 0.0, 1.0]));

	let v1 = black_box(Vector::from(vec![1.0, 2.0, 3.0]));
	let v2 = black_box(Vector::from(vec![0.0, 10.0, -100.0]));

    assert_eq!(Vector::from(vec![10.0, 0.0, 230.0]), Vector::linear_combination(&[v1, v2], &[10.0, -2.0]));

	c.bench_function("testoune", |b| b.iter(|| linear_combination([e1, e2, e3], [10.0, -2.0, 0.5])));
}

criterion_group!(benches, benchtest);
criterion_main!(benches);