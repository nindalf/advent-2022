use std::collections::HashSet;

#[inline]
pub fn part_1(input: &str) -> usize {
    let mut head = Point{x:0, y:0};
    let mut tail = Point{x:0, y:0};
    let mut tail_locations = HashSet::new();
    for movement in get_movement(input) {
        match movement {
            Movement::Left(steps) => {
                for _ in 0 .. steps {
                    head = Point{x:head.x-1, y:head.y};
                    tail = get_new_tail_position(&head, tail);
                    tail_locations.insert(tail);
                }
            },
            Movement::Right(steps) => {
                for _ in 0 .. steps {
                    head = Point{x:head.x+1, y:head.y};
                    tail = get_new_tail_position(&head, tail);
                    tail_locations.insert(tail);
                }
            },
            Movement::Down(steps) => {
                for _ in 0 .. steps {
                    head = Point{x:head.x, y:head.y-1};
                    tail = get_new_tail_position(&head, tail);
                    tail_locations.insert(tail);
                }
            },
            Movement::Up(steps) => {
                for _ in 0 .. steps {
                    head = Point{x:head.x, y:head.y+1};
                    tail = get_new_tail_position(&head, tail);
                    tail_locations.insert(tail);
                }
            },
        }
    }

    tail_locations.len()
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let mut knots = [Point{x:0, y:0}; 10];
    let mut tail_locations = HashSet::new();
    for movement in get_movement(input) {
        match movement {
            Movement::Left(steps) => {
                for _ in 0 .. steps {
                    knots[0] = Point{x:knots[0].x-1, y:knots[0].y};
                    for i in 1 .. 10 {
                        // println!("{i}, {:?}, {:?}", knots[i], knots[i-1]);
                        knots[i] = get_new_tail_position(&knots[i-1], knots[i]);
                    }
                    println!("Left {:?}", knots[9]);
                    tail_locations.insert(knots[9]);
                }
            },
            Movement::Right(steps) => {
                for _ in 0 .. steps {
                    knots[0] = Point{x:knots[0].x+1, y:knots[0].y};
                    for i in 1 .. 10 {
                        // println!("{i}, {:?}, {:?}", knots[i], knots[i-1]);
                        knots[i] = get_new_tail_position(&knots[i-1], knots[i]);
                    }
                    println!("Right {:?}", knots[9]);
                    tail_locations.insert(knots[9]);
                }
            },
            Movement::Down(steps) => {
                for _ in 0 .. steps {
                    knots[0] = Point{x:knots[0].x, y:knots[0].y-1};
                    for i in 1 .. 10 {
                        // println!("{i}, {:?}, {:?}", knots[i], knots[i-1]);
                        knots[i] = get_new_tail_position(&knots[i-1], knots[i]);
                    }
                    println!("Down {:?}", knots[9]);
                    tail_locations.insert(knots[9]);
                }
            },
            Movement::Up(steps) => {
                for _ in 0 .. steps {
                    knots[0] = Point{x:knots[0].x, y:knots[0].y+1};
                    for i in 1 .. 10 {
                        // println!("{i}, {:?}, {:?}", knots[i], knots[i-1]);
                        knots[i] = get_new_tail_position(&knots[i-1], knots[i]);
                    }
                    println!("Up {:?}", knots[9]);
                    tail_locations.insert(knots[9]);
                }
            },
        }
    }

    tail_locations.len()
}

fn get_new_tail_position(head:&Point, tail: Point) -> Point {
    
    let distance_x = i32::abs(head.x - tail.x);
    let distance_y = i32::abs(head.y - tail.y);

    let x = match (distance_x, distance_y) {
        (1, 1) | (0, 1) | (1, 0) | (0, 0) => tail,
        (2, 1) => Point{x:(head.x + tail.x)/2, y:head.y},
        (1, 2) => Point{x:head.x, y:(head.y + tail.y)/2},
        (2, 0) => Point{x:(head.x + tail.x)/2, y:tail.y},
        (0, 2) => Point{x:tail.x, y:(head.y + tail.y)/2},
        (2, 2) => Point{x:(head.x + tail.x)/2, y:(head.y + tail.y)/2},
        (x, y) => unreachable!("Tail is too far {x} {y}"),
    };
    x
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_movement(input: &str) -> impl Iterator<Item = Movement> + '_ {
    input
        .lines()
        .flat_map(|line| scan_fmt::scan_fmt!(line, "{[LRDU]} {d}", char, usize))
        .map(|(direction, steps)| match direction {
            'L' => Movement::Left(steps),
            'R' => Movement::Right(steps),
            'D' => Movement::Down(steps),
            'U' => Movement::Up(steps),
            c => panic!("Invalid direction {c}"),
        })
}

#[derive(Debug)]
enum Movement {
    Left(usize),
    Right(usize),
    Down(usize),
    Up(usize),
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("../inputs/day09-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day09.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 13);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 1234);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 36);
    }

    #[test]
    fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 1234);
    }

}
