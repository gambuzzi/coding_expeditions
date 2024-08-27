use y2023ex01::solve;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input: &str = include_str!("../input.txt");
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    c.bench_function("y2023ex01::part1", |b| {
        b.iter(|| solve(black_box(&lines), true))
    });
    c.bench_function("y2023ex01::part2", |b| {
        b.iter(|| solve(black_box(&lines), false))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
