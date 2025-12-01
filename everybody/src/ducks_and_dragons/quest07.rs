use crate::default_input_path;
use advent_of_code_common::Solver;
use std::collections::{HashMap, HashSet};

type Rules = HashMap<char, Vec<char>>;
type IndexedWords<'a> = Vec<(usize, &'a str)>;

fn parse_rules(input: &str) -> (IndexedWords<'_>, Rules) {
    let mut lines = input.lines();
    let words = lines.next().unwrap_or_default().split(',');
    let rules: Rules = lines
        .filter_map(|line| {
            let (first, second) = line.split_once('>')?;
            let first = first.trim().chars().next()?;
            let second = second.trim().chars().filter(|&c| c != ',').collect();
            Some((first, second))
        })
        .collect();
    let valid_words = words
        .into_iter()
        .enumerate()
        .filter(|(_, word)| {
            let chars: Vec<char> = word.chars().collect();
            for window in chars.windows(2) {
                if let [first, second] = window {
                    if let Some(rule) = rules.get(first) {
                        if !rule.contains(second) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            true
        })
        .collect();
    (valid_words, rules)
}

struct Part1;
impl<'a> Solver<'a> for Part1 {
    type Output = &'a str;

    fn solve(&self, input: &'a str) -> Self::Output {
        let (valid_words, _) = parse_rules(input);
        assert_eq!(valid_words.len(), 1);
        valid_words[0].1
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_rules(input).0.iter().map(|(i, _)| i + 1).sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

fn extend_word(word: &str, rules: &Rules, result: &mut Vec<String>) {
    if word.len() == 11 {
        return;
    }
    if let Some(rule) = word.chars().last().and_then(|c| rules.get(&c)) {
        for next_char in rule {
            let new_word = format!("{word}{next_char}");
            extend_word(&new_word, rules, result);
            result.push(new_word);
        }
    }
}

struct Part3;
impl Solver<'_> for Part3 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let (valid_words, rules) = parse_rules(input);
        let mut result = Vec::new();
        for (_, word) in valid_words {
            extend_word(word, &rules, &mut result);
        }
        result
            .into_iter()
            .filter(|w| w.len() >= 7 && w.len() <= 11)
            .collect::<HashSet<_>>()
            .len()
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
        expect_solution!(Part1, 0, "Oroneth");
        expect_solution!(Part1, 1, "Azadarin");
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 2, 23);
        expect_solution!(Part2, 3, 2353);
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 4, 25);
        expect_solution!(Part3, 5, 1154);
        expect_solution!(Part3, 6, 4736127);
    }
}
