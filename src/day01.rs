use std::collections::BinaryHeap;

#[allow(dead_code)]
fn max_calories(input: &str) -> i32 {
    let mut heap = heap_of_calories(input);
    heap.pop().unwrap_or_default()
}

#[allow(dead_code)]
fn max_calories_3(input: &str) -> i32 {
    let mut heap = heap_of_calories(input);
    heap.pop().unwrap_or_default() + heap.pop().unwrap_or_default() + heap.pop().unwrap_or_default()
}

fn heap_of_calories(input: &str) -> BinaryHeap<i32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .flat_map(|calorie| calorie.trim().parse::<i32>())
                .sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use anyhow::Result;

    static TEST_INPUT: &str = include_str!("../inputs/day01-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day01.txt");

    #[test]
    fn part_1_test() -> Result<()> {
        test(TEST_INPUT, &super::max_calories, 24000)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let start = Instant::now();
        test(FULL_INPUT, &super::max_calories, 70698)?;
        println!("Day 01 part 1 completed in {:?}", start.elapsed());
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(TEST_INPUT, &super::max_calories_3, 45000)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let start = Instant::now();
        test(FULL_INPUT, &super::max_calories_3, 206643)?;
        println!("Day 01 part 2 completed in {:?}", start.elapsed());
        Ok(())
    }

    fn test(input: &str, function: &dyn Fn(&str) -> i32, expected_val: i32) -> Result<()> {
        let result = function(input);
        assert_eq!(result, expected_val);
        Ok(())
    }
}
