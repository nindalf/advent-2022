use criterion::criterion_main;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

criterion_main!(
    // day01::benches,
    // day02::benches,
    // day03::benches,
    // day04::benches,
    // day05::benches,
    // day06::benches,
    // day07::benches,
    // day08::benches,
    // day09::benches,
    // day10::benches,
    // day11::benches,
    day12::benches,
);
