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

    static TEST_INPUT: &str = include_str!("../inputs/day01-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day01.txt");

    #[test]
    fn part_1_test() {
        let output = super::max_calories(TEST_INPUT);
        assert_eq!(output, 24000);
    }

    #[test]
    fn part_1() {
        let start = Instant::now();
        let output = super::max_calories(FULL_INPUT);
        assert_eq!(output, 70698);
        println!("Day 01 part 1 completed in {:?}", start.elapsed());
    }

    #[test]
    fn part_2_test() {
        let output = super::max_calories_3(TEST_INPUT);
        assert_eq!(output, 45000);
    }

    #[test]
    fn part_2() {
        let start = Instant::now();
        let output = super::max_calories_3(FULL_INPUT);
        assert_eq!(output, 206643);
        println!("Day 01 part 2 completed in {:?}", start.elapsed());
    }
}
