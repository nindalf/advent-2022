use std::collections::HashSet;

#[inline]
pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|backpack| backpack.split_at(backpack.len() / 2))
        .map(|(first, second)| find_common_item(&[first, second]))
        .map(|item| item.priority())
        .sum()
}

#[inline]
pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(find_common_item)
        .map(|item| item.priority())
        .sum()
}

fn find_common_item(backpacks: &[&str]) -> char {
    assert!(backpacks.len() > 1);
    let first = backpacks.first().unwrap().chars().collect::<HashSet<_>>();
    let intersection = backpacks
        .iter()
        .skip(1)
        .map(|backpack| backpack.chars().collect::<HashSet<_>>())
        .fold(first, |mut acc, backpack| {
            acc.retain(|c| backpack.contains(c));
            acc
        });

    assert_eq!(intersection.len(), 1);
    *intersection.iter().next().unwrap()
}

trait Priority {
    fn priority(self) -> i32;
}

impl Priority for char {
    fn priority(self) -> i32 {
        match self {
            c @ 'a'..='z' => (c as i32 - 'a' as i32) + 1,
            c @ 'A'..='Z' => (c as i32 - 'A' as i32) + 27,
            c => panic!("Cannot compute priority for {c}"),
        }
    }
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("../inputs/day03-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day03.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 157);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 7785);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 70);
    }

    #[test]
    pub fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 2633);
    }
}
