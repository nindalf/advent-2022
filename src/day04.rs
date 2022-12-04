use std::{str::FromStr, num::ParseIntError};

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    get_ranges(input)
        .filter(|(first, second)| {
            first.contains(second) || second.contains(first)
        })
        .count()
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    get_ranges(input)
        .filter(|(first, second)| {
            first.overlaps(second) || second.overlaps(first)
        })
        .count()
}

fn get_ranges(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    input.lines()
        .filter_map(|line| line.split_once(','))
        .flat_map(|(first, second)| {
            let first = first.parse::<Range>()?;
            let second = second.parse::<Range>()?;
            anyhow::Ok((first, second))
        })
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
        (self.end >= other.start && self.end <= other.start)
            || (self.start >= other.start && self.start <= other.end)
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseRangeError {
    #[error("not u32")]
    ParseInt(#[from] ParseIntError),
    #[error("failed to split at delimiter '-'")]
    ParseString,
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once('-').ok_or(ParseRangeError::ParseString)?;

        let start: u32 = first.parse()?;
        let end: u32 = second.parse()?;

        Ok(Range{start, end})
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    static TEST_INPUT: &str = include_str!("../inputs/day04-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day04.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 2);
    }

    #[test]
    fn part_1() {
        let start = Instant::now();
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 509);
        println!("Day 04 part 1 completed in {:?}", start.elapsed());
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 4);
    }

    #[test]
    fn part_2() {
        let start = Instant::now();
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 870);
        println!("Day 04 part 2 completed in {:?}", start.elapsed());
    }

}