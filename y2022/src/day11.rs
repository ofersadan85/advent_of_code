use advent_of_code_common::file::lines_as_blocks;
use anyhow::{Context, Result};

pub const PATH: &str = "../inputs/2022/day11.txt";
pub const EXAMPLE: &str = "../inputs/2022/day11_example.txt";

pub fn last_number(s: &str) -> Result<u128> {
    s.trim()
        .split_ascii_whitespace()
        .last()
        .context("Failed to find last number")?
        .parse()
        .context("Failed to parse last number")
}

#[derive(Debug, Clone)]
struct Monkey {
    holding: Vec<u128>,
    operation: String,
    divisor: u128,
    target_true: u128,
    target_false: u128,
    inspect_count: u128,
}

impl Monkey {
    fn from_block(block: &[String]) -> Result<Self> {
        let holding = block[1]
            .replace(',', "")
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let operation = block[2]
            .split(" = ")
            .last()
            .context("Failed to find operation in block")?
            .to_string();
        let divisor = last_number(&block[3])?;
        let target_true = last_number(&block[4])?;
        let target_false = last_number(&block[5])?;

        Ok(Self {
            holding,
            operation,
            divisor,
            target_true,
            target_false,
            inspect_count: 0,
        })
    }

    fn inspect(&self, item: u128) -> Result<u128> {
        if self.operation == "old * old" {
            Ok(item * item)
        } else {
            let number = last_number(&self.operation)?;
            match &self.operation[4..5] {
                "+" => Ok(item + number),
                "*" => Ok(item * number),
                _ => Err(anyhow::anyhow!("Invalid operation {}", self.operation)),
            }
        }
    }

    fn inspect_and_throw(&mut self, worry: bool) -> Result<Vec<(u128, u128)>> {
        let mut result = Vec::new();
        for item in &self.holding {
            let mut new_item = self.inspect(*item)?;
            if !worry {
                new_item /= 3;
            }
            self.inspect_count += 1;
            let target = if new_item % self.divisor == 0 {
                self.target_true
            } else {
                self.target_false
            };
            result.push((new_item, target));
        }
        self.holding = Vec::new();
        Ok(result)
    }
}

fn input(example: bool) -> Result<Vec<Monkey>> {
    let path = if example { EXAMPLE } else { PATH };
    let result =
        lines_as_blocks(&std::fs::read_to_string(path).context("Failed to read input file")?)
            .iter()
            .filter_map(|b| Monkey::from_block(b).ok())
            .collect();
    Ok(result)
}

fn throwing_round(monkeys: &mut [Monkey], worry: bool, max_div: u128) -> Result<()> {
    for i in 0..monkeys.len() {
        for (item, target) in monkeys[i].inspect_and_throw(worry)? {
            let item = item % max_div;
            let target_index: usize = target.try_into().context("Invalid target")?;
            monkeys[target_index].holding.push(item);
        }
    }
    Ok(())
}

fn part_1(monkeys: &mut [Monkey]) -> Result<u128> {
    let max_div: u128 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..20 {
        throwing_round(monkeys, false, max_div)?;
    }
    let mut inspect_counts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort_unstable();
    let result = inspect_counts[(monkeys.len() - 2)..monkeys.len()]
        .iter()
        .product();
    Ok(result)
}

fn part_2(monkeys: &mut [Monkey]) -> Result<u128> {
    let max_div: u128 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..10000 {
        throwing_round(monkeys, true, max_div)?;
    }
    let mut inspect_counts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort_unstable();
    let result = inspect_counts[(monkeys.len() - 2)..monkeys.len()]
        .iter()
        .product();
    Ok(result)
}

#[test]
fn example_1() {
    let mut monkeys = input(true).unwrap();
    assert_eq!(part_1(&mut monkeys).unwrap(), 10605);
}

#[test]
fn task_1() {
    let mut monkeys = input(false).unwrap();
    assert_eq!(part_1(&mut monkeys).unwrap(), 62491);
}

#[test]
fn example_2() {
    let mut monkeys = input(true).unwrap();
    assert_eq!(part_2(&mut monkeys).unwrap(), 2_713_310_158);
}

#[test]
fn task_2() {
    let mut monkeys = input(false).unwrap();
    assert_eq!(part_2(&mut monkeys).unwrap(), 17_408_399_184);
}
