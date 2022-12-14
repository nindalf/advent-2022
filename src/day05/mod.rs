#[inline]
pub fn part_1(input: &str) -> String {
    let (crates, raw_instructions) = input.split_once("\n\n").unwrap_or_default();
    let mut towers = get_towers(crates);

    for instruction in get_instructions(raw_instructions) {
        for _ in 0..instruction.n {
            match towers[instruction.origin].pop() {
                Some(popped) => towers[instruction.destination].push(popped),
                None => panic!("Attempted to pop from an empty tower"),
            }
        }
    }

    towers.iter().filter_map(|tower| tower.last()).collect()
}

#[inline]
pub fn part_2(input: &str) -> String {
    let (crates, raw_instructions) = input.split_once("\n\n").unwrap_or_default();
    let mut towers = get_towers(crates);

    for instruction in get_instructions(raw_instructions) {
        let origin_length = towers[instruction.origin].len();
        let to_move = towers[instruction.origin].split_off(origin_length - instruction.n);
        towers[instruction.destination].extend(to_move);
    }

    towers.iter().filter_map(|tower| tower.last()).collect()
}

fn get_towers(crates: &str) -> Vec<Vec<char>> {
    let num_towers = match crates.lines().next() {
        Some(line) => (line.len() + 1) / 4,
        None => 0,
    };
    let mut towers: Vec<Vec<char>> = vec![Vec::new(); num_towers];
    for line in crates.lines().rev() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'A'..='Z' => {
                    let tower_id = (i - 1) / 4;
                    towers[tower_id].push(c);
                }
                _ => continue,
            }
        }
    }
    towers
}

struct Instruction {
    n: usize,
    origin: usize,
    destination: usize,
}

fn get_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .flat_map(|line| scan_fmt::scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize))
        .map(|(n, origin, destination)| Instruction {
            n,
            origin: origin - 1,
            destination: destination - 1,
        })
}

crate::aoctest!("CMZ", "BZLVHBWQF", "MCD", "TDGJQTZSL");
