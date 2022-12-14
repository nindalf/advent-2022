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

crate::aoctest!(7, 1757, 19, 2950);
