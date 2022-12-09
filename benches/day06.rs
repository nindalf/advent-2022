use criterion::{black_box, criterion_group, Criterion};

static DAY_06_INPUT: &str = include_str!("../inputs/day06.txt");

fn bench_part_1(c: &mut Criterion) {
    c.bench_function("Day 06 Part 1", |b| {
        b.iter(|| advent_2022::day06::part_1(black_box(DAY_06_INPUT)))
    });
}

fn bench_part_2(c: &mut Criterion) {
    c.bench_function("Day 06 Part 2", |b| {
        b.iter(|| advent_2022::day06::part_2(black_box(DAY_06_INPUT)))
    });
}

criterion_group!(benches, bench_part_1, bench_part_2,);
