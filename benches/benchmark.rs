#[path = "../src/file_handler.rs"]
pub mod file_handler;

#[path = "../src/Y2022/mod.rs"]
pub mod Y2022;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day_01", |b| b.iter(|| crate::Y2022::days::day_01::Day01::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
