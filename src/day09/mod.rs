use std::collections::HashSet;

#[inline]
pub fn part_1(input: &str) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut tail_locations = HashSet::new();
    for movement in get_movement(input) {
        match &movement {
            Movement::Left(steps)
            | Movement::Right(steps)
            | Movement::Down(steps)
            | Movement::Up(steps) => {
                for _ in 0..*steps {
                    head = get_new_head_position(head, &movement);
                    tail = get_new_tail_position(&head, tail);
                    tail_locations.insert(tail);
                }
            }
        }
    }

    tail_locations.len()
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let mut knots = [Point { x: 0, y: 0 }; 10];
    let mut tail_locations = HashSet::new();
    for movement in get_movement(input) {
        match &movement {
            Movement::Left(steps)
            | Movement::Right(steps)
            | Movement::Down(steps)
            | Movement::Up(steps) => {
                for _ in 0..*steps {
                    knots[0] = get_new_head_position(knots[0], &movement);
                    for i in 1..10 {
                        knots[i] = get_new_tail_position(&knots[i - 1], knots[i]);
                    }
                    tail_locations.insert(knots[9]);
                }
            }
        }
    }

    tail_locations.len()
}

fn get_new_head_position(mut head: Point, movement: &Movement) -> Point {
    match movement {
        Movement::Left(_) => head.x -= 1,
        Movement::Right(_) => head.x += 1,
        Movement::Down(_) => head.y -= 1,
        Movement::Up(_) => head.y += 1,
    };
    head
}
fn get_new_tail_position(head: &Point, tail: Point) -> Point {
    let distance_x = i32::abs(head.x - tail.x);
    let distance_y = i32::abs(head.y - tail.y);

    match (distance_x, distance_y) {
        (1, 1) | (0, 1) | (1, 0) | (0, 0) => tail,
        (2, 1) => Point {
            x: (head.x + tail.x) / 2,
            y: head.y,
        },
        (1, 2) => Point {
            x: head.x,
            y: (head.y + tail.y) / 2,
        },
        (2, 0) => Point {
            x: (head.x + tail.x) / 2,
            y: tail.y,
        },
        (0, 2) => Point {
            x: tail.x,
            y: (head.y + tail.y) / 2,
        },
        (2, 2) => Point {
            x: (head.x + tail.x) / 2,
            y: (head.y + tail.y) / 2,
        },
        (x, y) => unreachable!("Tail is too far {x} {y}"),
    }
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
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 88);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 6384);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 36);
    }

    #[test]
    fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 2734);
    }
}
