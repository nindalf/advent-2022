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
    input //                                                            "1\n2\n3\n\n4\n\n5\n6\n"
        .lines() //                                                     ["1", "2", "3", "", "4", "", "5", "6"]
        .map(|line| line.trim().parse::<i32>()) //                      [Ok(1), Ok(2), Ok(3), ParseIntError, Ok(4), ParseIntError, Ok(5), Ok(6)]
        .collect::<Vec<_>>()
        .split(|num| num.is_err()) //                                   [[Ok(1), Ok(2), Ok(3)], [Ok(4)], [Ok(5), Ok(6)]]
        .map(|arr| arr.iter().filter_map(|i| i.as_ref().ok()).sum()) // [6, 4, 11]
        .collect() //                                                   BinaryHeap {data: [4, 6, 11]}
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
