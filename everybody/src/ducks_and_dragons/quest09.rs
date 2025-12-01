use std::collections::HashSet;

use crate::default_input_path;
use advent_of_code_common::Solver;
use itertools::Itertools;

#[expect(clippy::upper_case_acronyms)]
struct DNA<'a> {
    index: usize,
    sequence: &'a str,
}

impl<'a> DNA<'a> {
    fn from_input(input: &'a str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let (index, line) = line.split_once(':').unwrap_or_default();
                Self {
                    index: index.trim().parse().unwrap_or(0),
                    sequence: line,
                }
            })
            .collect()
    }

    fn similarity(&self, parent1: &Self, parent2: &Self) -> Option<usize> {
        let mut count1 = 0;
        let mut count2 = 0;
        for ((child, a), b) in self
            .sequence
            .chars()
            .zip(parent1.sequence.chars())
            .zip(parent2.sequence.chars())
        {
            let found = if child == a {
                count1 += 1;
                true
            } else if child == b {
                count2 += 1;
                true
            } else {
                false
            };
            if !found {
                return None;
            }
        }
        Some(count1 * count2)
    }
}

struct Part1And2;
impl Solver<'_> for Part1And2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let sequences = DNA::from_input(input);
        sequences
            .iter()
            .permutations(3)
            .filter(|seq| seq[1].index < seq[2].index) // Ensure unique parent pairs
            .filter_map(|seq| seq[0].similarity(seq[1], seq[2]))
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

fn merge_families(mut families: Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut distinct_families = Vec::new();
    while let Some(mut family) = families.pop() {
        if family.is_empty() {
            families.retain(|f| !f.is_empty());
            continue;
        }
        let mut merged = true;
        while merged {
            merged = false;
            for other_family in &mut families {
                if !family.is_disjoint(other_family) {
                    family.extend(other_family.iter());
                    other_family.clear();
                    merged = true;
                }
            }
        }
        if !merged {
            distinct_families.push(family);
        }
    }
    distinct_families
}

struct Part3;
impl Solver<'_> for Part3 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let sequences = DNA::from_input(input);
        let families: Vec<_> = sequences
            .iter()
            .permutations(3)
            .filter(|seq| seq[1].index < seq[2].index) // Ensure unique parent pairs
            .filter_map(|seq| {
                if seq[0].similarity(seq[1], seq[2]).is_some() {
                    Some(HashSet::from([seq[0].index, seq[1].index, seq[2].index]))
                } else {
                    None
                }
            })
            .collect();
        let groups = merge_families(families);
        groups
            .iter()
            .max_by_key(|g| g.len())
            .map(|g| g.iter().sum())
            .unwrap_or_default()
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1And2, 0, 414);
        expect_solution!(Part1And2, 1, 6512);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part1And2, 2, 1245);
        expect_solution!(Part1And2, 3, 316895);
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 4, 12);
        expect_solution!(Part3, 5, 36);
        expect_solution!(Part3, 6, 40528);
    }
}
