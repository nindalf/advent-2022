use rayon::prelude::*;
use rayon::str::ParallelString;

#[inline]
pub fn part_1(input: &str) -> usize {
    get_ranges(input)
        .filter(|(first, second)| first.contains(second) || second.contains(first))
        .count()
}

#[inline]
pub fn part_2(input: &str) -> usize {
    get_ranges(input)
        .filter(|(first, second)| first.overlaps(second))
        .count()
}

fn get_ranges(input: &str) -> impl ParallelIterator<Item = (Range, Range)> + '_ {
    input
        .par_lines()
        .flat_map(|line| scan_fmt::scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32))
        .map(|(a, b, x, y)| (Range { start: a, end: b }, Range { start: x, end: y }))
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.end >= other.start && self.end <= other.end)
            || (self.start >= other.start && self.start <= other.end)
            || self.contains(other)
    }
}

crate::aoctest!(2, 509, 4, 870);
