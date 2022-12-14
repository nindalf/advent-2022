#[inline]
pub fn part_1(input: &str) -> i32 {
    input.lines().map(strategy_one).sum()
}

#[inline]
pub fn part_2(input: &str) -> i32 {
    input.lines().map(strategy_two).sum()
}

fn strategy_one(line: &str) -> i32 {
    let opponent = line.chars().next().unwrap();
    let yours = line.chars().nth(2).unwrap();

    let basic_score = match yours {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    let fight_score = match (opponent, yours) {
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('C', 'Y') => 0,
        _ => 0,
    };

    basic_score + fight_score
}

fn strategy_two(line: &str) -> i32 {
    let opponent = line.chars().next().unwrap();
    let yours = line.chars().nth(2).unwrap();

    let basic_score = match (opponent, yours) {
        ('A', 'X') => 3,
        ('B', 'Y') => 2,
        ('C', 'Z') => 1,
        ('A', 'Y') => 1,
        ('B', 'Z') => 3,
        ('C', 'X') => 2,
        ('A', 'Z') => 2,
        ('B', 'X') => 1,
        ('C', 'Y') => 3,
        _ => 0,
    };

    let fight_score = match yours {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };

    basic_score + fight_score
}

crate::aoctest!(15, 14069, 12, 12411);
