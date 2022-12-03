use std::collections::HashSet;

#[allow(dead_code)]
fn part_1(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|backpack| backpack.split_at(backpack.len() / 2))
        .map(|(first, second)| common_item_priority(&[first.to_string(), second.to_string()]))
        .sum()
}

#[allow(dead_code)]
fn part_2(input: Vec<String>) -> i32 {
    input.chunks(3).map(common_item_priority).sum()
}

fn common_item_priority(backpacks: &[String]) -> i32 {
    assert!(backpacks.len() > 1);
    let first = backpacks.first().unwrap().chars().collect::<HashSet<_>>();
    let intersection = backpacks
        .iter()
        .skip(1)
        .map(|backpack| backpack.chars().collect::<HashSet<_>>())
        .fold(first, |acc, backpack| {
            acc.intersection(&backpack).copied().collect::<HashSet<_>>()
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
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day03-test.txt", &super::part_1, 157)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day03.txt", &super::part_1, 7785)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day03-test.txt", &super::part_2, 70)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day03.txt", &super::part_2, 2633)
    }

    fn test(
        test_file: &str,
        function: &dyn Fn(Vec<String>) -> i32,
        expected_val: i32,
    ) -> Result<()> {
        let input = crate::files::read_lines(test_file)?;
        let result = function(input);
        assert_eq!(result, expected_val);
        Ok(())
    }
}
