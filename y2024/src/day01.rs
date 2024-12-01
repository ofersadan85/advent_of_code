use anyhow::{anyhow, Result};

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut a = vec![];
    let mut b = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        a.push(parts.next().ok_or_else(|| anyhow!("missing a"))?.parse()?);
        b.push(parts.next().ok_or_else(|| anyhow!("missing b"))?.parse()?);
    }
    Ok((a, b))
}

fn part_1(a: &mut [u32], b: &mut [u32]) -> u32 {
    a.sort_unstable();
    b.sort_unstable();
    a.iter().zip(b.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

#[allow(clippy::cast_possible_truncation)]
fn part_2(a: &[u32], b: &[u32]) -> u32 {
    a.iter()
        .map(|a| a * b.iter().filter(|b| b == &a).count() as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const EXAMPLE: &str = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

    #[test]
    fn test_example_1() {
        let (mut a, mut b) = parse_input(EXAMPLE).unwrap();
        assert_eq!(part_1(&mut a, &mut b), 11);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2024/day01.txt").unwrap();
        let (mut a, mut b) = parse_input(&input).unwrap();
        assert_eq!(part_1(&mut a, &mut b), 1580061);
    }

    #[test]
    fn test_example_2() {
        let (a, b) = parse_input(EXAMPLE).unwrap();
        assert_eq!(part_2(&a, &b), 31);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2024/day01.txt").unwrap();
        let (a, b) = parse_input(&input).unwrap();
        assert_eq!(part_2(&a, &b), 23046913);
    }
}
