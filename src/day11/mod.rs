use std::collections::{VecDeque, HashMap};

#[inline]
pub fn part_1(mut monkeys: Vec<Monkey>) -> u128 {
    let mut inspections: HashMap<usize, u128> = HashMap::new();
    for round in 0 .. 20 {
        for i in 0 .. monkeys.len() {
            let mut destinations = vec![
                VecDeque::new(); monkeys.len()
            ];
            let monkey = &mut monkeys[i];
            // println!("{round} {i} {:?}", &monkey.items);
            while let Some(item) = monkey.items.pop_front() {
                let worry_level = (monkey.operation)(item)/3;
                let new_monkey = (monkey.test)(worry_level);
                destinations[new_monkey].push_back(worry_level);
                inspections.entry(i).and_modify(|count| *count += 1).or_insert(1);
            }
            for monkey in monkeys.iter_mut().rev() {
                monkey.items.extend(destinations.pop().unwrap());
            }
        }
    }
    let mut operations: Vec<&u128> = inspections.values().collect();
    operations.sort();
    operations[operations.len()-1] * operations[operations.len()-2]
}

#[inline]
pub fn part_2(mut monkeys: Vec<Monkey>) -> u128 {
    let mut inspections: HashMap<usize, u128> = HashMap::new();
    for round in 0 .. 10000 {
        for i in 0 .. monkeys.len() {
            let mut destinations = vec![
                VecDeque::new(); monkeys.len()
            ];
            let monkey = &mut monkeys[i];
            // println!("{round} {i} {:?}", &monkey.items);
            while let Some(item) = monkey.items.pop_front() {
                let worry_level = (monkey.worry_limiter)((monkey.operation)(item));
                let new_monkey = (monkey.test)(worry_level);
                destinations[new_monkey].push_back(worry_level);
                inspections.entry(i).and_modify(|count| *count += 1).or_insert(1);
            }
            for monkey in monkeys.iter_mut().rev() {
                monkey.items.extend(destinations.pop().unwrap());
            }
        }
    }
    let mut operations: Vec<&u128> = inspections.values().collect();
    operations.sort();
    operations[operations.len()-1] * operations[operations.len()-2]
}

pub struct Monkey {
    items: VecDeque<u128>,
    operation: Box<dyn Fn(u128)->u128>,
    test: Box<dyn Fn(u128)->usize>,
    worry_limiter: Box<dyn Fn(u128)->u128>
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::day11::Monkey;

    // static TEST_INPUT: &str = include_str!("test-input.txt");
    // static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let monkeys = vec![
            Monkey{
                items: VecDeque::from_iter(vec![79, 98]),
                operation: Box::new(|old| {old * 19}),
                test: Box::new(|old| {if old % 23 == 0 {2} else {3}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
            Monkey{
                items: VecDeque::from_iter(vec![54, 65, 75, 74]),
                operation: Box::new(|old| {old + 6}),
                test: Box::new(|old| {if old % 19 == 0 {2} else {0}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
            Monkey{
                items: VecDeque::from_iter(vec![79, 60, 97]),
                operation: Box::new(|old| {old * old}),
                test: Box::new(|old| {if old % 13 == 0 {1} else {3}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
            Monkey{
                items: VecDeque::from_iter(vec![74]),
                operation: Box::new(|old| old + 3),
                test: Box::new(|old| {if old % 17 == 0 {0} else {1}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
        ];
        let output = super::part_1(monkeys);
        assert_eq!(output, 10605);
    }

    #[test]
    fn part_1() {
        let monkeys = vec![
            Monkey{
                items: VecDeque::from_iter(vec![52, 78, 79, 63, 51, 94]),
                operation: Box::new(|old| old * 13),
                test: Box::new(|old| if old % 5 == 0 {1} else {6}),
                worry_limiter: Box::new(|old| if old % 5 == 0 {old/5} else {old}),
            },
            Monkey{
                items: VecDeque::from_iter(vec![77, 94, 70, 83, 53]),
                operation: Box::new(|old| old + 3),
                test: Box::new(|old| if old % 7 == 0 {5} else {3}),
                worry_limiter: Box::new(|old| if old % 7 == 0 {old/7} else {old}),
            },
            Monkey{
                items: VecDeque::from_iter(vec![98, 50, 76]),
                operation: Box::new(|old| old * old),
                test: Box::new(|old| if old % 13 == 0 {0} else {6}),
                worry_limiter: Box::new(|old| if old % 13 == 0 {old/13} else {old}),
            },
            Monkey{
                items: VecDeque::from_iter(vec![92, 91, 61, 75, 99, 63, 84, 69]),
                operation: Box::new(|old| old + 5),
                test: Box::new(|old| if old % 11 == 0 {5} else {7}),
                worry_limiter: Box::new(|old| if old % 11 == 0 {old/11} else {old}),
            },
            Monkey{
                items: VecDeque::from_iter(vec![51, 53, 83, 52]),
                operation: Box::new(|old| old + 7),
                test: Box::new(|old| if old % 3 == 0 {2} else {0}),
                worry_limiter: Box::new(|old| if old % 3 == 0 {old/3} else {old}),
            },
            Monkey{
                items: VecDeque::from_iter(vec![76, 76]),
                operation: Box::new(|old| old + 4),
                test: Box::new(|old| if old % 2 == 0 {4} else {7}),
                worry_limiter: Box::new(|old| if old % 2 == 0 {old/2} else {old}),
            },
            Monkey{
                items: VecDeque::from_iter(vec![75, 59, 93, 69, 76, 96, 65]),
                operation: Box::new(|old| old * 19),
                test: Box::new(|old| if old % 17 == 0 {1} else {3}),
                worry_limiter: Box::new(|old| if old % 17 == 0 {old/17} else {old}),
            },
            Monkey{
                items: VecDeque::from_iter(vec![89]),
                operation: Box::new(|old| old + 2),
                test: Box::new(|old| if old % 19 == 0 {2} else {4}),
                worry_limiter: Box::new(|old| if old % 19 == 0 {old/19} else {old}),
            },
        ];
        let output = super::part_1(monkeys);
        assert_eq!(output, 58786);
    }

    #[test]
    fn part_2_test() {
        let monkeys = vec![
            Monkey{
                items: VecDeque::from_iter(vec![79, 98]),
                operation: Box::new(|old| {old * 19}),
                test: Box::new(|old| {if old % 23 == 0 {2} else {3}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
            Monkey{
                items: VecDeque::from_iter(vec![54, 65, 75, 74]),
                operation: Box::new(|old| {old + 6}),
                test: Box::new(|old| {if old % 19 == 0 {2} else {0}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
            Monkey{
                items: VecDeque::from_iter(vec![79, 60, 97]),
                operation: Box::new(|old| {old * old}),
                test: Box::new(|old| {if old % 13 == 0 {1} else {3}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
            Monkey{
                items: VecDeque::from_iter(vec![74]),
                operation: Box::new(|old| old + 3),
                test: Box::new(|old| {if old % 17 == 0 {0} else {1}}),
                worry_limiter: Box::new(|old| old % 96577),
            },
        ];
        let output = super::part_2(monkeys);
        assert_eq!(output, 2713310158);
    }

    #[test]
    fn part_2() {
        let monkeys = vec![
            Monkey{
                items: VecDeque::from_iter(vec![52, 78, 79, 63, 51, 94]),
                operation: Box::new(|old| old * 13),
                test: Box::new(|old| if old % 5 == 0 {1} else {6}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
            Monkey{
                items: VecDeque::from_iter(vec![77, 94, 70, 83, 53]),
                operation: Box::new(|old| old + 3),
                test: Box::new(|old| if old % 7 == 0 {5} else {3}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
            Monkey{
                items: VecDeque::from_iter(vec![98, 50, 76]),
                operation: Box::new(|old| old * old),
                test: Box::new(|old| if old % 13 == 0 {0} else {6}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
            Monkey{
                items: VecDeque::from_iter(vec![92, 91, 61, 75, 99, 63, 84, 69]),
                operation: Box::new(|old| old + 5),
                test: Box::new(|old| if old % 11 == 0 {5} else {7}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
            Monkey{
                items: VecDeque::from_iter(vec![51, 53, 83, 52]),
                operation: Box::new(|old| old + 7),
                test: Box::new(|old| if old % 3 == 0 {2} else {0}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
            Monkey{
                items: VecDeque::from_iter(vec![76, 76]),
                operation: Box::new(|old| old + 4),
                test: Box::new(|old| if old % 2 == 0 {4} else {7}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
            Monkey{
                items: VecDeque::from_iter(vec![75, 59, 93, 69, 76, 96, 65]),
                operation: Box::new(|old| old * 19),
                test: Box::new(|old| if old % 17 == 0 {1} else {3}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
            Monkey{
                items: VecDeque::from_iter(vec![89]),
                operation: Box::new(|old| old + 2),
                test: Box::new(|old| if old % 19 == 0 {2} else {4}),
                worry_limiter: Box::new(|old| old % 9699690),
            },
        ];
        let output = super::part_2(monkeys);
        assert_eq!(output, 14952185856);
    }

}
