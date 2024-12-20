use advent_of_code_macros::aoc_tests;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn part_1(input: &str) -> usize {
    let mut result = String::new();
    for row in input.lines() {
        let (a, b) = row.trim().split_at(row.trim().len() / 2);
        let char_count_a: HashMap<char, usize> = a.chars().counts();
        let char_count_b: HashMap<char, usize> = b.chars().counts();
        for c in char_count_a.keys() {
            if char_count_b.contains_key(c) {
                result.push(*c);
            }
        }
    }
    result
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c as usize - 96
            } else {
                c as usize - 38
            }
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let data: Vec<&str> = input.lines().collect();
    (0..data.len())
        .step_by(3)
        .map(|i| {
            data[i]
                .trim()
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&data[i + 1].trim().chars().collect())
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&data[i + 2].trim().chars().collect())
                .map(|c| {
                    if c.is_ascii_lowercase() {
                        *c as usize - 96
                    } else {
                        *c as usize - 38
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 157);
    }

    #[test]
    fn solution_1() {
        assert_eq!(part_1(&read_input()), 7428);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 70);
    }

    #[test]
    fn solution_2() {
        assert_eq!(part_2(&read_input()), 2650);
    }
}
