use aoc_2023::day1::{self};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1 part 1", |b| {
        b.iter(|| day1::part1(black_box(include_str!("../input/day_1_1.txt"))));
    });

    c.bench_function("day1 part 2", |b| {
        b.iter(|| day1::part2(black_box(include_str!("../input/day_1_1.txt"))));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
