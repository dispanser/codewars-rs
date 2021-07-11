use codewars::consecutive_strings::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark_longest_original(c: &mut Criterion) {
    let n = 100;
    let content = "falafelpopafel";
    let inputs: Vec<&str> = std::iter::repeat(content).take(n).collect();
    c.bench_function("original manual loop", |b| {
        b.iter(|| longest_consec(inputs.clone(), n / 2))
    });
}

pub fn benchmark_longest_windowing(c: &mut Criterion) {
    let n = 100;
    let content = "falafelpopafel";
    let inputs: Vec<&str> = std::iter::repeat(content).take(n).collect();
    c.bench_function("windowing on strings", |b| {
        b.iter(|| longest_windowing(inputs.clone(), n / 2))
    });
}

pub fn benchmark_longest_windowing_lengths(c: &mut Criterion) {
    let n = 100;
    let content = "falafelpopafel";
    let inputs: Vec<&str> = std::iter::repeat(content).take(n).collect();
    c.bench_function("windowing on lengths", |b| {
        b.iter(|| longest_windowing_lengths(inputs.clone(), n / 2))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = benchmark_longest_original, benchmark_longest_windowing, benchmark_longest_windowing_lengths
}

criterion_main!(benches);
