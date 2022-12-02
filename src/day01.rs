use std::collections::BinaryHeap;

#[allow(dead_code)]
fn max_calories(input: String) -> i32 {
    let mut heap = heap_of_calories(input);
    heap.pop().unwrap_or_default()
}

#[allow(dead_code)]
fn max_calories_3(input: String) -> i32 {
    let mut heap = heap_of_calories(input);
    heap.pop().unwrap_or_default() + heap.pop().unwrap_or_default() + heap.pop().unwrap_or_default()
}

fn heap_of_calories(input: String) -> BinaryHeap<i32> {
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
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day01-test.txt", &super::max_calories, 24000)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day01.txt", &super::max_calories, 70698)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day01-test.txt", &super::max_calories_3, 45000)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day01.txt", &super::max_calories_3, 206643)
    }

    fn test(test_file: &str, function: &dyn Fn(String) -> i32, expected_val: i32) -> Result<()> {
        let input = crate::files::read_string(test_file)?;
        let result = function(input);
        assert_eq!(result, expected_val);
        Ok(())
    }
}
