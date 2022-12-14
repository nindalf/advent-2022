use std::collections::BinaryHeap;

#[inline]
pub fn part_1(input: &str) -> i32 {
    let mut heap = heap_of_calories(input);
    heap.pop().unwrap_or_default()
}

#[inline]
pub fn part_2(input: &str) -> i32 {
    let mut heap = heap_of_calories(input);
    heap.pop().unwrap_or_default() + heap.pop().unwrap_or_default() + heap.pop().unwrap_or_default()
}

fn heap_of_calories(input: &str) -> BinaryHeap<i32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .flat_map(|calorie| calorie.trim().parse::<i32>())
                .sum()
        })
        .collect()
}

crate::aoctest!(24000, 70698, 45000, 206643);
