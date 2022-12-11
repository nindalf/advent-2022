use criterion::{black_box, criterion_group, Criterion};

static DAY_11_INPUT: &str = include_str!("../src/day11/input.txt");

fn bench_part_1(c: &mut Criterion) {
    c.bench_function("Day 11 Part 1", |b| {
        b.iter(|| advent_2022::day11::part_1(black_box(DAY_11_INPUT)))
    });
}

fn bench_part_2(c: &mut Criterion) {
    c.bench_function("Day 11 Part 2", |b| {
        b.iter(|| advent_2022::day11::part_2(black_box(DAY_11_INPUT)))
    });
}

criterion_group!(benches, bench_part_1, bench_part_2,);
