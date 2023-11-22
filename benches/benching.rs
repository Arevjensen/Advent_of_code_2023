use aoc_2023::day1::{self};
use aoc_2023::helpers;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    const DAY: &str = "1";
    const PART: &str = "1";

    c.bench_function("day1 part 1 load file", |b| {
        b.iter(|| day1::part1(black_box(&helpers::loader::read_text_from_file(PART, DAY))));
    });

    c.bench_function("day1 part 1 include str", |b| {
        b.iter(|| day1::part1(black_box(include_str!("../input/day_1_1.txt"))));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
