use std::cell::RefCell;

#[inline]
pub fn part_1(input: &str) -> u64 {
    let monkeys = get_monkeys(input);
    simulate(monkeys, 20, &|item| item / 3)
}

#[inline]
pub fn part_2(input: &str) -> u64 {
    let monkeys = get_monkeys(input);
    let modulo: u64 = monkeys.iter().map(|m| m.divisible_by).product();
    simulate(monkeys, 10000, &|item| item % modulo)
}

fn simulate(monkeys: Vec<Monkey>, rounds: u64, worry_limiter: &dyn Fn(u64) -> u64) -> u64 {
    let mut operations = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.borrow().iter() {
                let item = match monkeys[i].operation {
                    Op::Add(arg) => item + arg,
                    Op::Mul(arg) => item * arg,
                    Op::Square => item * item,
                };
                let limited_item = (worry_limiter)(item);
                let destination = if limited_item % monkeys[i].divisible_by == 0 {
                    monkeys[i].dest_monkey_true
                } else {
                    monkeys[i].dest_monkey_false
                };
                monkeys[destination].items.borrow_mut().push(limited_item);
                operations[i] += 1;
            }
            monkeys[i].items.borrow_mut().clear();
        }
    }
    operations.sort();
    let l = operations.len();
    operations[l - 1] * operations[l - 2]
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| {
            let lines: Vec<_> = m.lines().skip(1).map(|l| l.trim()).collect();
            let items = lines[0][16..]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
            let operation = if lines[1] == "Operation: new = old * old" {
                Op::Square
            } else if lines[1].starts_with("Operation: new = old * ") {
                Op::Mul(lines[1]["Operation: new = old * ".len()..].parse().unwrap())
            } else {
                Op::Add(lines[1]["Operation: new = old + ".len()..].parse().unwrap())
            };
            let divisible_by = lines[2]["Test: divisible by ".len()..].parse().unwrap();
            let dest_monkey_true = lines[3]["If true: throw to monkey ".len()..]
                .parse()
                .unwrap();
            let dest_monkey_false = lines[4]["If false: throw to monkey ".len()..]
                .parse()
                .unwrap();
            Monkey {
                items: RefCell::new(items),
                operation,
                divisible_by,
                dest_monkey_true,
                dest_monkey_false,
            }
        })
        .collect()
}

#[derive(Clone)]
pub struct Monkey {
    items: RefCell<Vec<u64>>,
    operation: Op,
    divisible_by: u64,
    dest_monkey_true: usize,
    dest_monkey_false: usize,
}

#[derive(Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

crate::aoctest!(10605, 58786, 2713310158, 14952185856);
