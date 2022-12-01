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
    input //                                              "1000\n2000\n3000\n\n4000\n\n5000\n6000\n"
        .lines() //                                       ["1000", "2000", "3000", "", "4000", "", "5000", "6000"]
        .map(|line| line.trim().parse::<i32>()) //        [Ok(1000), Ok(2000), Ok(3000), ParseIntError, Ok(4000), ParseIntError, Ok(5000), Ok(6000)]
        .collect::<Vec<_>>()
        .split(|num| num.is_err()) //                     [[Ok(1000), Ok(2000), Ok(3000)], [Ok(4000)], [Ok(5000), Ok(6000)]]
        .map(|arr| arr.iter().filter_map(|i| i.as_ref().ok()).sum()) // [6000, 4000, 11000]
        .collect() //                                      BinaryHeap {data: [4000, 6000, 11000]}
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day1-test.txt", &super::max_calories, 24000)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day1.txt", &super::max_calories, 70698)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day1-test.txt", &super::max_calories_3, 45000)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day1.txt", &super::max_calories_3, 206643)
    }

    fn test(test_file: &str, function: &dyn Fn(String) -> i32, expected_val: i32) -> Result<()> {
        let input = crate::files::read_string(test_file)?;
        let result = function(input);
        assert_eq!(result, expected_val);
        Ok(())
    }
}
