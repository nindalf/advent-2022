use criterion::{black_box, criterion_group, Criterion};

static DAY_09_INPUT: &str = include_str!("../inputs/day09.txt");

fn bench_part_1(c: &mut Criterion) {
    c.bench_function("Day 09 Part 1", |b| {
        b.iter(|| advent_2022::day09::part_1(black_box(DAY_09_INPUT)))
    });
}

fn bench_part_2(c: &mut Criterion) {
    c.bench_function("Day 09 Part 2", |b| {
        b.iter(|| advent_2022::day09::part_2(black_box(DAY_09_INPUT)))
    });
}

criterion_group!(benches, bench_part_1, bench_part_2,);
