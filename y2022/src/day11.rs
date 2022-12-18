use advent_of_code_common::file::lines_as_blocks;

pub const PATH: &str = "inputs/day11.txt";
pub const EXAMPLE: &str = "inputs/day11_example.txt";

pub fn last_number(s: &str) -> u128 {
    s.trim()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Monkey {
    holding: Vec<u128>,
    operation: String,
    divisor: u128,
    target_true: usize,
    target_false: usize,
    inspect_count: u128,
}

impl Monkey {
    fn from_block(block: &[String]) -> Self {
        let holding = block[1]
            .replace(',', "")
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let operation = block[2].split(" = ").last().unwrap().to_string();
        let divisor = last_number(&block[3]);
        let target_true = last_number(&block[4]).try_into().unwrap();
        let target_false = last_number(&block[5]).try_into().unwrap();

        Self {
            holding,
            operation,
            divisor,
            target_true,
            target_false,
            inspect_count: 0,
        }
    }

    fn inspect(&self, item: u128) -> u128 {
        if self.operation == "old * old" {
            item * item
        } else {
            let number = last_number(&self.operation);
            match &self.operation[4..5] {
                "+" => item + number,
                "*" => item * number,
                _ => panic!("Unknown operation found {}", self.operation),
            }
        }
    }

    fn inspect_and_throw(&mut self, worry: bool) -> Vec<(u128, usize)> {
        let mut result = Vec::new();
        for item in &self.holding {
            let mut new_item = self.inspect(*item);
            if !worry {
                new_item /= 3;
            };
            self.inspect_count += 1;
            let target = if new_item % self.divisor == 0 {
                self.target_true
            } else {
                self.target_false
            };
            result.push((new_item, target));
        }
        self.holding = Vec::new();
        result
    }
}

fn input(example: bool) -> Vec<Monkey> {
    let path = if example { EXAMPLE } else { PATH };
    lines_as_blocks(&std::fs::read_to_string(path).unwrap())
        .iter()
        .map(|b| Monkey::from_block(b))
        .collect()
}

fn throwing_round(monkeys: &mut [Monkey], worry: bool, max_div: u128) {
    for i in 0..monkeys.len() {
        for (item, target) in monkeys[i].inspect_and_throw(worry) {
            let item = item % max_div;
            monkeys[target].holding.push(item);
        }
    }
}

fn part_1(monkeys: &mut [Monkey]) -> u128 {
    let max_div: u128 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..20 {
        throwing_round(monkeys, false, max_div);
    }
    let mut inspect_counts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort_unstable();
    inspect_counts[(monkeys.len() - 2)..monkeys.len()]
        .iter()
        .product()
}

fn part_2(monkeys: &mut [Monkey]) -> u128 {
    let max_div: u128 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..10000 {
        throwing_round(monkeys, true, max_div);
    }
    let mut inspect_counts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort_unstable();
    inspect_counts[(monkeys.len() - 2)..monkeys.len()]
        .iter()
        .product()
}

#[test]
fn example_1() {
    let mut monkeys = input(true);
    assert_eq!(part_1(&mut monkeys), 10605);
}

#[test]
fn task_1() {
    let mut monkeys = input(false);
    assert_eq!(part_1(&mut monkeys), 62491);
}

#[test]
fn example_2() {
    let mut monkeys = input(true);
    assert_eq!(part_2(&mut monkeys), 2_713_310_158);
}

#[test]
fn task_2() {
    let mut monkeys = input(false);
    assert_eq!(part_2(&mut monkeys), 17_408_399_184);
}
