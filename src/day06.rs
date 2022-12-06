use std::collections::HashSet;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let window_size = 4;
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .filter_map(|(i, x)| {
            let set = x.iter().copied().collect::<HashSet<char>>();
            if set.len() == window_size {
                return Some(i + window_size);
            }
            None
        })
        .next()
        .unwrap()
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    let window_size = 14;
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .filter_map(|(i, x)| {
            let set = x.iter().copied().collect::<HashSet<char>>();
            if set.len() == window_size {
                return Some(i + window_size);
            }
            None
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    static TEST_INPUT: &str = include_str!("../inputs/day06-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day06.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 7);
    }

    #[test]
    fn part_1() {
        let start = Instant::now();
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 1757);
        println!("Day 06 part 1 completed in {:?}", start.elapsed());
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 19);
    }

    #[test]
    fn part_2() {
        let start = Instant::now();
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 2950);
        println!("Day 06 part 2 completed in {:?}", start.elapsed());
    }
}
