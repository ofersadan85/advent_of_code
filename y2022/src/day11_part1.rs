use std::ops::{Add, Mul};

use advent_of_code_common::{file::lines_as_blocks, math::prime_factors};

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
enum Item {
    Value(u128),
    Factors(Vec<u128>),
}

impl Item {
    fn from_number(n: u128) -> Self {
        Self::Factors(prime_factors(&n))
    }

    fn from_str(s: &str) -> Option<Self> {
        let n = s.parse().ok()?;
        Some(Self::Factors(prime_factors(&n)))
    }

    fn value(&self) -> u128 {
        match self {
            Self::Value(n) => *n,
            Self::Factors(factors) => factors.iter().product(),
        }
    }

    fn to_factors(&self) -> Self {
        match self {
            Item::Value(n) => Self::from_number(*n),
            Item::Factors(_) => self.clone(),
        }
    }
}

impl Add for Item {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if let Self::Factors(_) = self {
            let new_value = self.value() + rhs.value();
            let new_factors = prime_factors(&new_value);
            Self::Factors(new_factors)
        } else {
            Self::Value(self.value() + rhs.value())
        }
    }
}

impl Mul for Item {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if let Self::Factors(lhs_factors) = self {
            let mut lhs_factors = lhs_factors;
            match rhs {
                Item::Value(n) => {
                    lhs_factors.push(n);
                }
                Item::Factors(rhs_factors) => {
                    lhs_factors.extend(rhs_factors);
                }
            }
            Self::Factors(lhs_factors)
        } else {
            Self::Value(self.value() * rhs.value())
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    holding: Vec<Item>,
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
            .filter_map(Item::from_str)
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

    fn inspect(&self, item: Item) -> Item {
        if self.operation == "old * old" {
            item.clone() * item
        } else {
            let number = Item::Value(last_number(&self.operation));
            match &self.operation[4..5] {
                "+" => item + number,
                "*" => item * number,
                _ => panic!("Unknown operation found {}", self.operation),
            }
        }
    }

    fn inspect_and_throw(&mut self, worry: bool) -> Vec<(Item, usize)> {
        let mut result = Vec::new();
        for item in &self.holding {
            let new_item = if worry {
                self.inspect(item.clone())
            } else {
                Item::Value(self.inspect(item.clone()).value() / 3)
            };
            self.inspect_count += 1;
            if let Item::Factors(factors) = new_item.to_factors() {
                // Should always be true
                let target = if factors.contains(&self.divisor) {
                    self.target_true
                } else {
                    self.target_false
                };
                result.push((new_item, target));
            }
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

fn throwing_round(monkeys: &mut [Monkey], worry: bool) {
    for i in 0..monkeys.len() {
        for (item, target) in monkeys[i].inspect_and_throw(worry) {
            monkeys[target].holding.push(item);
        }
    }
}

fn part_1(monkeys: &mut [Monkey]) -> u128 {
    for _ in 0..20 {
        throwing_round(monkeys, false);
    }
    let mut inspect_counts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort_unstable();
    inspect_counts[(monkeys.len() - 2)..monkeys.len()]
        .iter()
        .product()
}

fn part_2(monkeys: &mut [Monkey]) -> u128 {
    for _ in 0..10000 {
        throwing_round(monkeys, true);
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
