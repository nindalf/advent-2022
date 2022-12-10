use nom::{character::complete::{newline, multispace1}, IResult, bytes::complete::tag, sequence::{pair, separated_pair, terminated}, combinator::{map, recognize}, branch::alt};

#[inline]
pub fn part_1(input: &str) -> i32 {
    let computer = Computer{input};
    let mut cycle_number = 0;
    let mut result = 0;
    let mut register_x = 1;
    for instruction in computer {
        cycle_number += match instruction {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 1,
        };
        if (cycle_number - 20) % 40 == 0 {
            result += cycle_number * register_x;
        }
        if let Instruction::Addx(_) = instruction {
            cycle_number += 1;
        } else {
            continue;
        }
        if (cycle_number - 20) % 40 == 0 {
            result += cycle_number * register_x;
        }

        match instruction {
            Instruction::Noop => {},
            Instruction::Addx(arg) => {
                register_x += arg;
            },
        }
    }
    result
}

#[inline]
pub fn part_2(input: &str) -> String {
    let computer = Computer{input};
    let mut cycle_number = 0;
    let mut result = vec![];
    let mut register_x = 1;
    
    for instruction in computer {
        cycle_number += match instruction {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 1,
        };
        if overlap(cycle_number, register_x) {
            result.push('#');
        } else {
            result.push('.');
        }
        if cycle_number % 40 == 0{
            result.push('\n');
        }
        if let Instruction::Addx(_) = instruction {
            cycle_number += 1;
        } else {
            continue;
        }
        if overlap(cycle_number, register_x) {
            result.push('#');
        } else {
            result.push('.');
        }
        if cycle_number % 40 == 0{
            result.push('\n');
        }
        
        match instruction {
            Instruction::Noop => {},
            Instruction::Addx(arg) => {
                register_x += arg;
            },
        }
    }
    result.iter().collect()
}

fn overlap(cycle_number: i32, register_x: i32) -> bool {
    let pixel = (cycle_number - 1) % 40;
    pixel == register_x - 1 || pixel == register_x || pixel == register_x + 1
}
enum Instruction {
    Noop,
    Addx(i32),
}

struct Computer<'a> {
    input: &'a str,
}

impl<'a> Iterator for Computer<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        match next_instruction(self.input) {
            Ok((remaining, instruction)) => {
                self.input = remaining;
                Some(instruction)
            }
            Err(e) => panic!("Unexpected error {e}"),
        }
    }
}

fn next_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((noop, addx))(input)
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    map(recognize(pair(tag("noop"), newline)), |s: &str| {
        Instruction::Noop
    })(input)
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    map(
        terminated(
            separated_pair(tag("addx"), multispace1, nom::character::complete::i32),
            newline,
        ),
        |(_, argument): (_, i32)| Instruction::Addx(argument),
    )(input)
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
