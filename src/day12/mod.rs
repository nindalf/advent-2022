use std::collections::{BinaryHeap, HashMap, VecDeque};

use crate::matrix::{Matrix, Point};

#[inline]
pub fn part_1(input: &str) -> u32 {
    let matrix = get_matrix(input);
    let start = matrix.find(0).next().unwrap();
    let end_val = 27;
    let step_allowed =
        |current_elevation, neighbour_elevation| current_elevation + 1 >= neighbour_elevation;
    matrix.bfs_cost(&start, end_val, &step_allowed)
}

#[inline]
pub fn part_2(input: &str) -> u32 {
    let matrix = get_matrix(input);
    let start = matrix.find(27).next().unwrap();
    let end_val = 1;
    let step_allowed =
        |current_elevation, neighbour_elevation| current_elevation - 1 <= neighbour_elevation;
    matrix.bfs_cost(&start, end_val, &step_allowed)
}

fn get_matrix(input: &str) -> Matrix {
    let storage = input
        .chars()
        .filter_map(|c| match c {
            'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
            'S' => Some(0),
            'E' => Some(27),
            '\n' => None,
            x => unreachable!("Invalid input {x}"),
        })
        .collect::<Vec<_>>();
    let max_x = input.lines().next().unwrap().chars().count();
    let max_y = input.split_ascii_whitespace().count();
    Matrix {
        storage,
        max_x,
        max_y,
    }
}

impl Matrix {
    // I've specifically implemented BFS for day 12 because it's 10% faster
    // To convert bfs() to dijkstra() make these changes
    // 1. Rename queue to heap
    // 2. Change VecDeque to BinaryHeap
    // 3. Change push_back() to push() and pop_front() to pop()
    fn bfs_cost(
        &self,
        start: &Point,
        end_val: u32,
        step_allowed: &dyn Fn(u32, u32) -> bool,
    ) -> u32 {
        let mut queue = VecDeque::with_capacity(100);
        let mut visited = HashMap::new();
        let start = State::new(*start, 0);
        queue.push_back(start);

        while let Some(state) = queue.pop_front() {
            if *self.value(&state.point).unwrap() == end_val {
                return state.cost;
            }
            for neighbour in self.neighbours(state.point) {
                if let Some(value) = self.value(&neighbour) {
                    if !step_allowed(*self.value(&state.point).unwrap(), *value) {
                        continue;
                    }
                    let cost_to_neighbour = state.cost + 1; // constant cost 1 for this problem
                    if let Some(previous_cost) = visited.get(&neighbour) {
                        // Already a better route to this neigbour
                        // Ignore this neighbour and move onto the next one
                        if *previous_cost <= cost_to_neighbour {
                            continue;
                        }
                    }
                    queue.push_back(State {
                        point: neighbour,
                        cost: cost_to_neighbour,
                    });
                    visited.insert(neighbour, cost_to_neighbour);
                }
            }
        }
        unreachable!("Must find the end")
    }
}

// Copied from the example on https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    point: Point,
    cost: u32,
}

impl State {
    fn new(point: Point, cost: u32) -> State {
        State { point, cost }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Notice that the we flip the ordering on costs.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 31);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 484);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 29);
    }

    #[test]
    fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 478);
    }
}
