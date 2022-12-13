use std::cmp::Ordering;

use serde_json::{json, Value};

#[inline]
pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|pairs| pairs.split_once('\n'))
        .map(|(left, right)| {
            let left = serde_json::from_str(left).unwrap();
            let right = serde_json::from_str(right).unwrap();
            compare(&left, &right)
        })
        .enumerate()
        .filter_map(|(i, ordering)| {
            if ordering == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let mut values: Vec<Value> = input.lines().flat_map(serde_json::from_str).collect();

    values.extend([
        serde_json::from_str("[[2]]").unwrap(),
        serde_json::from_str("[[6]]").unwrap(),
    ]);
    values.sort_by(compare);

    let packet_one: Value = serde_json::from_str("[[2]]").unwrap();
    let position_one = values.iter().position(|val| *val == packet_one).unwrap() + 1;

    let packet_two: Value = serde_json::from_str("[[6]]").unwrap();
    let position_two = values.iter().position(|val| *val == packet_two).unwrap() + 1;
    
    position_one * position_two
}

fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(x), Value::Number(y)) => x.as_u64().cmp(&y.as_u64()),
        (Value::Number(x), Value::Array(y)) => compare(&json!([x]), &json!(y)),
        (Value::Array(x), Value::Number(y)) => compare(&json!(x), &json!([y])),
        (Value::Array(x), Value::Array(y)) => {
            for i in 0..usize::min(x.len(), y.len()) {
                let result = compare(&x[i], &y[i]);
                if result != Ordering::Equal {
                    return result;
                }
            }
            // All elements equal so far.
            // Now compare on length
            x.len().cmp(&y.len())
        }
        (x, y) => unreachable!("Unsupported json values {x} {y}"),
    }
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 13);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 5580);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 140);
    }

    #[test]
    fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 26200);
    }
}
