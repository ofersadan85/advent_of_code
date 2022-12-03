use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn input(example: bool) -> Vec<String> {
    const PATH: &str = "inputs/day03.txt";
    if example {
        "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .split('\n')
    .map(String::from)
    .collect()
}

fn part_1(data: &[String]) -> usize {
    let mut result = String::new();
    for row in data {
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

fn part_2(data: &[String]) -> usize {
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

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 157);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 7428);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 70);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 2650);
}
