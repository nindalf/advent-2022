use std::collections::HashSet;

#[inline]
pub fn part_1(input: &str) -> usize {
    let rocks = get_rocks(input);
    let bottom_layer = *rocks.iter().map(|(_, y)| y).max().unwrap();
    let mut sand = HashSet::new();
    while let Some(spot) = next_sand_spot((500, 0), &rocks, &sand, bottom_layer) {
        sand.insert(spot);
    }
    sand.len()
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let mut rocks = get_rocks(input);
    let bottom_layer = *rocks.iter().map(|(_, y)| y).max().unwrap() + 2;
    let max_x = *rocks.iter().map(|(x, _)| x).max().unwrap() + 1000;
    (0..=max_x)
        .zip(std::iter::repeat(bottom_layer))
        .for_each(|p| {
            rocks.insert(p);
        });
    let mut sand = HashSet::new();
    while let Some(spot) = next_sand_spot((500, 0), &rocks, &sand, bottom_layer + 1) {
        sand.insert(spot);
        if spot == (500, 0) {
            break;
        }
    }
    sand.len()
}

fn next_sand_spot(
    spot: (u32, u32),
    rocks: &HashSet<(u32, u32)>,
    sand: &HashSet<(u32, u32)>,
    bottom_layer: u32,
) -> Option<(u32, u32)> {
    if spot.1 >= bottom_layer {
        // We've equalled the bottom layer. No hope.
        return None;
    }
    let down = (spot.0, spot.1 + 1);
    if !rocks.contains(&down) && !sand.contains(&down) {
        return next_sand_spot(down, rocks, sand, bottom_layer);
    }
    let down_left = (spot.0 - 1, spot.1 + 1);
    if !rocks.contains(&down_left) && !sand.contains(&down_left) {
        return next_sand_spot(down_left, rocks, sand, bottom_layer);
    }
    let down_right = (spot.0 + 1, spot.1 + 1);
    if !rocks.contains(&down_right) && !sand.contains(&down_right) {
        return next_sand_spot(down_right, rocks, sand, bottom_layer);
    }
    // Can't move further, and we're not below the bottom layer.
    // This is the permanent spot
    Some(spot)
}

fn get_rocks(input: &str) -> HashSet<(u32, u32)> {
    let rocks = input
        .lines()
        .flat_map(|line| {
            let pairs: Vec<(u32, u32)> = line
                .split(" -> ")
                .filter_map(|pair| pair.split_once(','))
                .filter_map(|(x, y)| x.parse::<u32>().ok().zip(y.parse::<u32>().ok()))
                .collect();
            let points: Vec<_> = pairs
                .clone()
                .into_iter()
                .zip(pairs.into_iter().skip(1))
                .flat_map(|((x, y), (a, b))| {
                    if x == a {
                        return std::iter::repeat(x)
                            .zip(u32::min(y, b)..=u32::max(y, b))
                            .collect::<Vec<_>>();
                    }
                    (u32::min(x, a)..=u32::max(x, a))
                        .zip(std::iter::repeat(y))
                        .collect::<Vec<_>>()
                })
                .collect();
            points
        })
        .collect::<HashSet<(u32, u32)>>();
    rocks
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 24);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 618);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 93);
    }

    #[test]
    fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 26358);
    }
}
