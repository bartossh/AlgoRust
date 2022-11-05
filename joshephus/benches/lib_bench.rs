use criterion::{black_box, criterion_group, criterion_main, Criterion};
use joshephus::solver;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("joshephus 251", |b| b.iter(|| solver(black_box(251))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

