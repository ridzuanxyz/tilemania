use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_kwg_load(c: &mut Criterion) {
    c.bench_function("load CSW24.kwg", |b| {
        b.iter(|| {
            let bytes = fs::read("assets/lexicons/CSW24.kwg").unwrap();
            black_box(bytes.len());
        });
    });
}

criterion_group!(benches, benchmark_kwg_load);
criterion_main!(benches);
