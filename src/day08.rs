use crate::matrix::{Matrix, Point};
use take_until::TakeUntilExt;

#[inline]
pub fn part_1(input: &str) -> usize {
    let matrix = Matrix::new(input);

    matrix
        .all_points()
        .filter(|point| {
            let tree_val = matrix.value(point);

            let invisible_right = (point.x + 1..matrix.max_x)
                .map(|x| Point { x, y: point.y })
                .any(|p| matrix.value(&p) >= tree_val);

            let invisible_left = (0..point.x)
                .rev()
                .map(|x| Point { x, y: point.y })
                .any(|p| matrix.value(&p) >= tree_val);

            let invisible_bottom = (point.y + 1..matrix.max_y)
                .map(|y| Point { x: point.x, y })
                .any(|p| matrix.value(&p) >= tree_val);

            let invisible_top = (0..point.y)
                .rev()
                .map(|y| Point { x: point.x, y })
                .any(|p| matrix.value(&p) >= tree_val);

            !invisible_right || !invisible_left || !invisible_bottom || !invisible_top
        })
        .count()
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let matrix = Matrix::new(input);

    matrix
        .all_points()
        .map(|point| {
            let tree_val = matrix.value(&point);

            let trees_right = (point.x + 1..matrix.max_x)
                .map(|x| Point { x, y: point.y })
                .take_until(|p| matrix.value(p) >= tree_val)
                .count();

            let trees_left = (0..point.x)
                .rev()
                .map(|x| Point { x, y: point.y })
                .take_until(|p| matrix.value(p) >= tree_val)
                .count();

            let trees_bottom = (point.y + 1..matrix.max_y)
                .map(|y| Point { x: point.x, y })
                .take_until(|p| matrix.value(p) >= tree_val)
                .count();

            let trees_top = (0..point.y)
                .rev()
                .map(|y| Point { x: point.x, y })
                .take_until(|p| matrix.value(p) >= tree_val)
                .count();

            // scenic score
            trees_right * trees_left * trees_bottom * trees_top
        })
        .max()
        .unwrap()
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
        assert_eq!(output, 268912);
    }
}
