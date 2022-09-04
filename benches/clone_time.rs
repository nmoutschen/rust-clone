use criterion::{criterion_group, criterion_main};
use rust_clone::benchmark;

criterion_group!(benches, benchmark);
criterion_main!(benches);
