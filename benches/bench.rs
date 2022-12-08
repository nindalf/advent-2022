#[macro_use]
extern crate criterion;
use criterion::{Criterion, black_box};


static DAY_01_INPUT: &str = include_str!("../inputs/day01.txt");

static DAY_02_INPUT: &str = include_str!("../inputs/day02.txt");

static DAY_03_INPUT: &str = include_str!("../inputs/day03.txt");

static DAY_04_INPUT: &str = include_str!("../inputs/day04.txt");

static DAY_05_INPUT: &str = include_str!("../inputs/day05.txt");

static DAY_06_INPUT: &str = include_str!("../inputs/day06.txt");

static DAY_07_INPUT: &str = include_str!("../inputs/day07.txt");

static DAY_08_INPUT: &str = include_str!("../inputs/day08.txt");



fn bench_01(c: &mut Criterion) {
    c.bench_function("Day 01 Part 1", |b| b.iter(|| advent_2022::day01::part_1(black_box(DAY_01_INPUT))));
    c.bench_function("Day 01 Part 2", |b| b.iter(|| advent_2022::day01::part_2(black_box(DAY_01_INPUT))));
}

fn bench_02(c: &mut Criterion) {
    c.bench_function("Day 02 Part 1", |b| b.iter(|| advent_2022::day02::part_1(black_box(DAY_02_INPUT))));
    c.bench_function("Day 02 Part 2", |b| b.iter(|| advent_2022::day02::part_2(black_box(DAY_02_INPUT))));
}

fn bench_03(c: &mut Criterion) {
    c.bench_function("Day 03 Part 1", |b| b.iter(|| advent_2022::day03::part_1(black_box(DAY_03_INPUT))));
    c.bench_function("Day 03 Part 2", |b| b.iter(|| advent_2022::day03::part_2(black_box(DAY_03_INPUT))));
}

fn bench_04(c: &mut Criterion) {
    c.bench_function("Day 04 Part 1", |b| b.iter(|| advent_2022::day04::part_1(black_box(DAY_04_INPUT))));
    c.bench_function("Day 04 Part 2", |b| b.iter(|| advent_2022::day04::part_2(black_box(DAY_04_INPUT))));
}

fn bench_05(c: &mut Criterion) {
    c.bench_function("Day 05 Part 1", |b| b.iter(|| advent_2022::day05::part_1(black_box(DAY_05_INPUT))));
    c.bench_function("Day 05 Part 2", |b| b.iter(|| advent_2022::day05::part_2(black_box(DAY_05_INPUT))));
}

fn bench_06(c: &mut Criterion) {
    c.bench_function("Day 06 Part 1", |b| b.iter(|| advent_2022::day06::part_1(black_box(DAY_06_INPUT))));
    c.bench_function("Day 06 Part 2", |b| b.iter(|| advent_2022::day06::part_2(black_box(DAY_06_INPUT))));
}

fn bench_07(c: &mut Criterion) {
    c.bench_function("Day 07 Part 1", |b| b.iter(|| advent_2022::day07::part_1(black_box(DAY_07_INPUT))));
    c.bench_function("Day 07 Part 2", |b| b.iter(|| advent_2022::day07::part_2(black_box(DAY_07_INPUT))));
}

fn bench_08(c: &mut Criterion) {
    c.bench_function("Day 08 Part 1", |b| b.iter(|| advent_2022::day08::part_1(black_box(DAY_08_INPUT))));
    c.bench_function("Day 08 Part 2", |b| b.iter(|| advent_2022::day08::part_2(black_box(DAY_08_INPUT))));
}


criterion_group!(
    benches, 
    
    bench_01,
    
    bench_02,
    
    bench_03,
    
    bench_04,
    
    bench_05,
    
    bench_06,
    
    bench_07,
    
    bench_08,
    
);
criterion_main!(benches);
