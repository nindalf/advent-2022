use crate::computer::Computer;

#[inline]
pub fn part_1(input: &str) -> i32 {
    let mut computer = Computer::new(input);
    let mut result = 0;
    while let Some(state) = computer.next_cycle() {
        if (state.cycle_counter - 20) % 40 == 0 {
            result += state.cycle_counter * state.register_x;
        }
    }
    result
}

#[inline]
pub fn part_2(input: &str) -> String {
    let mut computer = Computer::new(input);
    let mut result = vec![];
    while let Some(state) = computer.next_cycle() {
        result.push(state.pixel);
        if state.cycle_counter % 40 == 0 {
            result.push('\n');
        }
    }
    result.iter().collect()
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 13140);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 14520);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(output, expected);
    }

    #[test]
    fn part_2() {
        let output = super::part_2(FULL_INPUT);
        let expected = "###..####.###...##..####.####...##.###..
#..#....#.#..#.#..#....#.#.......#.#..#.
#..#...#..###..#......#..###.....#.###..
###...#...#..#.#.##..#...#.......#.#..#.
#....#....#..#.#..#.#....#....#..#.#..#.
#....####.###...###.####.####..##..###..
";
        assert_eq!(output, expected);
    }
}
