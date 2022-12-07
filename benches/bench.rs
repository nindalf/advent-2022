#[macro_use]
extern crate criterion;
use criterion::Criterion;

fn bench_method1(c: &mut Criterion) {
    max_calories
}

fn bench_method2(c: &mut Criterion) {}

criterion_group!(benches, bench_method1, bench_method2);
criterion_main!(benches);
