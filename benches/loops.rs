use criterion::{black_box, criterion_group, criterion_main, Criterion};

use loop_scan_bench::cases::a;

fn bench_a(c: &mut Criterion) {
    c.bench_function("a::fusion", |b| b.iter(|| a::fusion(black_box(10_000))));
    c.bench_function("a::fission", |b| b.iter(|| a::fission(black_box(10_000))));
}

criterion_group!(benches, bench_a);
criterion_main!(benches);