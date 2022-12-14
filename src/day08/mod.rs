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

crate::aoctest!(21, 1803, 8, 268912);

