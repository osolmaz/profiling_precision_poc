use criterion::{black_box, criterion_group, criterion_main, Criterion};
use profiling_precision_poc::empty;
use profiling_precision_poc::fibonacci;
use profiling_precision_poc::generate_random_u64;
use profiling_precision_poc::increment_u128;
use rand::rngs::ThreadRng;
// use rand::Rng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut a = 0;
    let mut rng: ThreadRng = rand::thread_rng();

    c.bench_function("increment_u128", |b| b.iter(|| increment_u128(&mut a)));
    c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("generate_random_u64", |b| {
        b.iter(|| generate_random_u64(&mut rng))
    });
    c.bench_function("empty", |b| b.iter(|| empty()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
