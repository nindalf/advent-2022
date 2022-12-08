use criterion::{black_box, criterion_group, Criterion};

static DAY_01_INPUT: &str = include_str!("../inputs/day01.txt");

fn bench_part_1(c: &mut Criterion) {
    c.bench_function("Day 01 Part 1", |b| {
        b.iter(|| advent_2022::day01::part_1(black_box(DAY_01_INPUT)))
    });
}

fn bench_part_2(c: &mut Criterion) {
    c.bench_function("Day 01 Part 2", |b| {
        b.iter(|| advent_2022::day01::part_2(black_box(DAY_01_INPUT)))
    });
}

criterion_group!(
    benches, 
    bench_part_1,
    bench_part_2,
);
