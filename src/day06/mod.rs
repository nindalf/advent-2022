#[inline]
pub fn part_1(input: &str) -> usize {
    find_first_window_with_unique_chars(input, 4)
}

#[inline]
pub fn part_2(input: &str) -> usize {
    find_first_window_with_unique_chars(input, 14)
}

fn find_first_window_with_unique_chars(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .position(|x| {
            let mut chars = x.to_vec();
            chars.sort();
            chars.dedup();
            chars.len() == window_size
        })
        .unwrap()
        + window_size
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 7);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 1757);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 19);
    }

    #[test]
    pub fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 2950);
    }
}
