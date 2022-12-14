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
crate::aoctest!(13140, 14520, 
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
", 
"###..####.###...##..####.####...##.###..
#..#....#.#..#.#..#....#.#.......#.#..#.
#..#...#..###..#......#..###.....#.###..
###...#...#..#.#.##..#...#.......#.#..#.
#....#....#..#.#..#.#....#....#..#.#..#.
#....####.###...###.####.####..##..###..
");
