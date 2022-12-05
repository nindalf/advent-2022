#[allow(dead_code)]
fn part_1(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap_or_default();
    let mut towers = get_towers(crates);
    for line in instructions.lines() {
        let (n, origin, destination) = scan_fmt::scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        for _ in 0 .. n {
            match towers[origin].pop() {
                Some(popped) => towers[destination].push(popped),
                None => panic!("Attempted to pop from an empty tower"),
            }
        }
    }
    towers.iter().skip(1).filter_map(|tower| tower.last()).collect::<String>()
}

#[allow(dead_code)]
fn part_2(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap_or_default();
    let mut towers = get_towers(crates);
    for line in instructions.lines() {
        let (n, origin, destination) = scan_fmt::scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        let mut temp = vec![];
        for _ in 0 .. n {
            match towers[origin].pop() {
                Some(popped) => temp.push(popped),
                None => panic!("Attempted to pop from an empty tower"),
            }
        }
        for c in temp.iter().rev() {
            towers[destination].push(*c);
        }
    }
    towers.iter().skip(1).filter_map(|tower| tower.last()).collect::<String>()
}

fn get_towers(crates: &str) -> Vec<Vec<char>> {
    let num_towers = (crates.lines().next().unwrap().len() / 4) + 2;
    let mut towers: Vec<Vec<char>> = vec![Vec::new(); num_towers];
    for line in crates.lines().rev() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'A' ..= 'Z' => {
                    let tower_id = ((i-1)/4) + 1;
                    towers[tower_id].push(c);
                }
                _ => continue,
            }
        }
    }
    towers
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    static TEST_INPUT: &str = include_str!("../inputs/day05-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day05.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, "CMZ");
    }

    #[test]
    fn part_1() {
        let start = Instant::now();
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, "BZLVHBWQF");
        println!("Day 05 part 1 completed in {:?}", start.elapsed());
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, "MCD");
    }

    #[test]
    fn part_2() {
        let start = Instant::now();
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, "TDGJQTZSL");
        println!("Day 05 part 2 completed in {:?}", start.elapsed());
    }

}
