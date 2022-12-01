#[path = "../src/file_handler.rs"]
pub mod file_handler;

#[path = "../src/days/mod.rs"]
pub mod days;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("Day_01", |b| b.iter(|| days::day_1::Day01::run()));
   
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
