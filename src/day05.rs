#[allow(dead_code)]
fn part_1(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut towers = get_towers(crates);
    for line in instructions.lines() {
        let (n, origin, destination) = scan_fmt::scan_fmt!(line, "move {d} from {d} to {d}", u32, usize, usize).unwrap();
        for _ in 0 .. n {
            let x = towers[origin].pop().unwrap();
            towers[destination].push(x);
        }
    }
    let result = towers.iter().skip(1).filter_map(|tower| tower.last()).collect::<String>();
    result
}

#[allow(dead_code)]
fn part_2(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut towers = get_towers(crates);
    for line in instructions.lines() {
        let (n, origin, destination) = scan_fmt::scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        let mut temp = vec![];
        for _ in 0 .. n {
            let x = towers[origin].pop().unwrap();
            temp.push(x);
        }
        for c in temp.iter().rev() {
            towers[destination].push(*c);
        }
    }
    let result = towers.iter().skip(1).filter_map(|tower| tower.last()).collect::<String>();
    result
}

fn get_towers(crates: &str) -> Vec<Vec<char>> {
    let mut towers: Vec<Vec<char>> = vec![Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),];
    for line in crates.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'A' ..= 'Z' => {
                    let tower_id = match i {
                        1 => 1, // 1 => 0
                        5 => 2, // 5 => 1
                        9 => 3, // 9 => 2
                        13 => 4, // 13 => 3
                        17 => 5,
                        21 => 6,
                        25 => 7,
                        29 => 8,
                        33 => 9,
                        _ => panic!("Unsupported index"),
                    };
                    towers[tower_id].push(c);
                }
                _ => continue,
            }
        }
    }
    towers = towers.iter().map(|tower| tower.iter().copied().rev().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
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
