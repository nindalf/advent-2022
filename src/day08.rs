use crate::matrix::{Matrix, Point};

#[inline]
pub fn part_1(input: &str) -> usize {
    let matrix = Matrix::new(input);
    println!("{}", matrix);
    let mut trees = vec![Tree::default(); matrix.len()];

    for x in 0..matrix.max_x {
        let point = Point{x, y: 0};
        trees[matrix.idx(&point)].visible_from_left = true;
        let mut max_so_far = matrix.value(&point).unwrap();
        for y in 0 .. matrix.max_y {
            let point = Point{x, y};
            let value = matrix.value(&point).unwrap();
            if max_so_far < value {
                max_so_far = value;
                println!("Set {}, {} to true", x, y);
                trees[matrix.idx(&point)].visible_from_left = true;
            }
        }
    }
    for x in (0..matrix.max_x) {
        let point = Point{x, y: matrix.max_y-1};
        trees[matrix.idx(&point)].visible_from_right = true;
        let mut max_so_far = matrix.value(&point).unwrap();
        for y in (0 .. matrix.max_y).rev() {
            let point = Point{x, y};
            let value = matrix.value(&point).unwrap();
            if max_so_far < value {
                max_so_far = value;
                println!("Set {}, {} to visible from right", x, y);
                trees[matrix.idx(&point)].visible_from_right = true;
            }
        }
    }
    for y in 0..matrix.max_y {
        let point = Point{x:0, y};
        trees[matrix.idx(&point)].visible_from_top = true;
        let mut max_so_far = matrix.value(&point).unwrap();
        for x in 0 .. matrix.max_x {
            let point = Point{x, y};
            let value = matrix.value(&point).unwrap();
            if max_so_far < value {
                max_so_far = value;
                trees[matrix.idx(&point)].visible_from_top = true;
            }
        }
    }
    for y in (0..matrix.max_y) {
        let point = Point{x:matrix.max_x-1, y};
        trees[matrix.idx(&point)].visible_from_bottom = true;
        let mut max_so_far = matrix.value(&point).unwrap();
        for x in (0 .. matrix.max_x).rev() {
            let point = Point{x, y};
            let value = matrix.value(&point).unwrap();
            if max_so_far < value {
                max_so_far = value;
                trees[matrix.idx(&point)].visible_from_bottom = true;
            }
        }
    }
    trees.iter().filter(|tree| tree.visible()).count()
}

#[derive(Default, Clone, Debug)]
struct Tree {
    visible_from_top: bool,
    visible_from_left: bool,
    visible_from_right: bool,
    visible_from_bottom: bool,
    trees_top: usize,
    trees_left: usize,
    trees_right: usize,
    trees_bottom: usize,
}

impl Tree {
    fn visible(&self) -> bool {
        self.visible_from_top || self.visible_from_left || self.visible_from_right || self.visible_from_bottom
    }

    fn scenic_score(&self) -> usize {
        self.trees_top * self.trees_left * self.trees_right * self.trees_bottom
    }
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let matrix = Matrix::new(input);
    let mut trees = vec![Tree::default(); matrix.len()];

    for point in matrix.all_points() {
        let tree_idx = matrix.idx(&point);
        for x in point.x+1 .. matrix.max_x {
            let new_point = Point{x, y: point.y};
            trees[tree_idx].trees_right += 1;
            if matrix.value(&new_point) >= matrix.value(&point) {
                break
            }
        }

        for x in (0 .. point.x).rev() {
            let new_point = Point{x, y: point.y};
            trees[tree_idx].trees_left += 1;
            if matrix.value(&new_point) >= matrix.value(&point) {
                break
            }
        }

        for y in point.y+1 .. matrix.max_y {
            let new_point = Point{x: point.x, y};
            trees[tree_idx].trees_bottom += 1;
            if matrix.value(&new_point) >= matrix.value(&point) {
                break
            }
        }

        for y in (0 .. point.y).rev() {
            let new_point = Point{x: point.x, y};
            trees[tree_idx].trees_top += 1;
            if matrix.value(&new_point) >= matrix.value(&point) {
                break
            }
        }
    }
    trees.iter().map(|tree| tree.scenic_score()).max().unwrap()
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("../inputs/day08-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day08.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 21);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 1803);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 8);
    }

    #[test]
    fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 1234);
    }

}
