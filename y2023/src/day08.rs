use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

pub const EXAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

pub const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

pub const EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn prime_factors(n: &u128) -> Vec<u128> {
    let mut n = *n;
    let mut div = 2;
    let mut result = Vec::new();
    let max_div = n.isqrt();
    while n > 1 {
        if div > max_div {
            result.push(n);
            break;
        } else if n.is_multiple_of(div) {
            result.push(div);
            n /= div;
            div = 2;
        } else {
            div += 1;
        }
    }
    result
}

fn parse_line(s: &str) -> Result<(&str, (&str, &str))> {
    let mut parts = s.split(" = ");
    let key = parts.next().context("no key")?;
    let value = parts.next().context("no value")?;
    let mut parts = value.split(", ");
    let left = parts.next().context("no left")?.trim_start_matches('(');
    let right = parts.next().context("no right")?.trim_end_matches(')');
    Ok((key, (left, right)))
}

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

#[allow(clippy::implicit_hasher)]
fn parse_input(s: &str) -> Result<(&str, Map<'_>)> {
    let mut lines = s.lines();
    let commands = lines.next().context("no commands")?;
    let map = lines.filter_map(|line| parse_line(line).ok()).collect();
    Ok((commands, map))
}

#[allow(clippy::implicit_hasher)]
fn solve1(commands: &str, map: &Map) -> u128 {
    let mut count = 0;
    let mut key = "AAA";
    let mut commands = commands.chars().cycle();
    while key != "ZZZ" {
        let (left, right) = map.get(key).expect("bad key");
        key = match commands.next() {
            Some('L') => left,
            Some('R') => right,
            _ => unreachable!(),
        };
        count += 1;
    }
    count
}

#[allow(clippy::implicit_hasher)]
fn solve2(commands: &str, map: &Map) -> u128 {
    let mut count = 0;
    let mut keys: Vec<_> = map.keys().filter(|&&key| key.ends_with('A')).collect();
    let mut commands = commands.chars().cycle();
    let mut z_map = vec![vec![]; keys.len()];
    while z_map.iter().any(|v| v.len() < 2) {
        let current_command = commands.next();
        let next_keys: Vec<_> = keys
            .iter()
            .map(|&&key| {
                let (left, right) = map.get(key).expect("bad key");
                match current_command {
                    Some('L') => left,
                    Some('R') => right,
                    _ => unreachable!(),
                }
            })
            .collect();
        next_keys.iter().enumerate().for_each(|(i, &key)| {
            if key.ends_with('Z') {
                z_map[i].push(count);
            }
        });
        keys = next_keys;
        count += 1;
    }
    let prime_set: HashSet<u128> = z_map
        .into_iter()
        .flat_map(|v| prime_factors(&(v[1] - v[0])))
        .collect();
    prime_set.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let (commands, map) = parse_input(EXAMPLE1).unwrap();
        assert_eq!(solve1(commands, &map), 2);
    }

    #[test]
    fn example2() {
        let (commands, map) = parse_input(EXAMPLE2).unwrap();
        assert_eq!(solve1(commands, &map), 6);
    }

    #[test]
    fn part1() {
        let (commands, map) = parse_input(include_str!("../../inputs/2023/day08.txt")).unwrap();
        assert_eq!(solve1(commands, &map), 17873);
    }

    #[test]
    fn example3() {
        let (commands, map) = parse_input(EXAMPLE3).unwrap();
        let factors = solve2(commands, &map);
        assert_eq!(factors, 6);
    }

    #[test]
    fn part2() {
        let (commands, map) = parse_input(include_str!("../../inputs/2023/day08.txt")).unwrap();
        let factors = solve2(commands, &map);
        assert_eq!(factors, 15_746_133_679_061);
    }
}
