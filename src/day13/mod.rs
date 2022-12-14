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
        (Value::Number(x), Value::Array(_)) => compare(&json!([x]), right),
        (Value::Array(_), Value::Number(y)) => compare(left, &json!([y])),
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

crate::aoctest!(13, 5580, 140, 26200);
