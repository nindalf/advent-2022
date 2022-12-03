use std::collections::HashSet;

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|backpack| backpack.split_at(backpack.len() / 2))
        .map(|(first, second)| common_item_priority(&[first, second]))
        .sum()
}

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(common_item_priority)
        .sum()
}

fn common_item_priority(backpacks: &[&str]) -> i32 {
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
    let common_item = *intersection.iter().next().unwrap();
    match common_item {
        'a'..='z' => (common_item as i32 - 'a' as i32) + 1,
        'A'..='Z' => (common_item as i32 - 'A' as i32) + 27,
        _ => panic!("Invalid char"),
    }
}
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use anyhow::Result;

    static TEST_INPUT: &str = include_str!("../inputs/day03-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day03.txt");

    #[test]
    fn part_1_test() -> Result<()> {
        test(TEST_INPUT, &super::part_1, 157)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let start = Instant::now();
        test(FULL_INPUT, &super::part_1, 7785)?;
        println!("Day 03 part 1 completed in {:?}", start.elapsed());
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(TEST_INPUT, &super::part_2, 70)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let start = Instant::now();
        test(FULL_INPUT, &super::part_2, 2633)?;
        println!("Day 03 part 2 completed in {:?}", start.elapsed());
        Ok(())
    }

    fn test(input: &str, function: &dyn Fn(&str) -> i32, expected_val: i32) -> Result<()> {
        let result = function(input);
        assert_eq!(result, expected_val);
        Ok(())
    }
}
