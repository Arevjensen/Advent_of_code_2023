use aoc_2023::day05::{self};
use aoc_2023::helpers;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_function("day5 part 2 load file", |b| {
        b.iter(|| day05::part2(black_box(&helpers::loader::read_text_from_file("1", "5"))));
    });

    group.finish();
    // c.bench_function("day1 part 1 include str", |b| {
    //     b.iter(|| day05::part2(black_box(include_str!("../input/day_5_1_big.txt"))));
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
