use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::*;

pub fn bench_euler_1(c: &mut Criterion) {
    c.bench_function("Euler 1",
                     |b| b.iter(|| euler_1::main()));
}

pub fn bench_euler_2(c: &mut Criterion) {
    c.bench_function("Euler 2",
                     |b| b.iter(|| euler_2::main()));
}

criterion_group!(benches, bench_euler_1, bench_euler_2);
criterion_main!(benches);
