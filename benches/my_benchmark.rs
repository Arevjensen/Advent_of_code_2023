use aoc_2023::day1::{self};
use aoc_2023::helpers;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    //Alternative use include_str("../input/day_1_1.txt") to not bench file parsing/loading
    c.bench_function("day1 part 1", |b| {
        b.iter(|| day1::part1(black_box(&helpers::loader::read_text_from_file("1", "1"))));
    });

    c.bench_function("day1 part 2", |b| {
        b.iter(|| day1::part2(black_box(&helpers::loader::read_text_from_file("1", "1"))));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
