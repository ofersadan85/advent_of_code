use advent_of_code_common::grid::{Direction, Grid};
use colored::Colorize;
use itertools::Itertools;
use std::collections::HashSet;

fn extract_runes(input: &str) -> Vec<&str> {
    input
        .lines()
        .next()
        .expect("runes line")
        .trim_start_matches("WORDS:")
        .split(',')
        .collect()
}

fn clean_input(input: &str) -> String {
    let rune_chars: HashSet<char> = extract_runes(input)
        .iter()
        .flat_map(|r| r.chars())
        .collect();
    let other_chars: HashSet<char> = input
        .lines()
        .skip(1)
        .flat_map(|l| l.chars())
        .filter(|c| !c.is_whitespace())
        .collect::<HashSet<_>>()
        .difference(&rune_chars)
        .copied()
        .collect();
    let mut clean_input = input.lines().skip(1).join("\n");
    for c in other_chars {
        clean_input = clean_input.replace(c, " ");
    }
    clean_input.trim_start().to_string()
}

fn count_runes(input: &str) -> usize {
    let runes = extract_runes(input);
    let words = clean_input(input);
    let mut count = 0;
    for w in words.split_whitespace() {
        for r in &runes {
            let mut w = w;
            while !w.is_empty() {
                if let Some(i) = w.find(r) {
                    count += 1;
                    w = &w[i + r.len()..];
                } else {
                    break;
                }
            }
        }
    }
    count
}

fn count_rune_symbols(input: &str) -> usize {
    let runes = extract_runes(input);
    let reverse_runes: Vec<String> = runes.iter().map(|r| r.chars().rev().collect()).collect();
    let runes: Vec<String> = runes
        .iter()
        .map(ToString::to_string)
        .chain(reverse_runes)
        .collect();
    let words = clean_input(input);
    let mut count = 0;
    for w in words.split_whitespace() {
        let mut indexes = HashSet::new();
        for r in &runes {
            let mut w = w;
            let mut prev = 0;
            while !w.is_empty() {
                if let Some(i) = w.find(r) {
                    indexes.extend(i + prev..i + prev + r.len());
                    w = &w[i + 1..];
                    prev += i + 1;
                } else {
                    break;
                }
            }
        }
        count += indexes.len();
    }
    count
}

#[derive(Debug)]
struct GridCell {
    c: char,
    is_rune: std::cell::Cell<bool>,
}

impl From<char> for GridCell {
    fn from(c: char) -> Self {
        Self {
            c,
            is_rune: std::cell::Cell::new(false),
        }
    }
}

impl std::fmt::Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.is_rune.get() {
            self.c.to_string().red()
        } else {
            self.c.to_string().normal()
        };
        write!(f, "{c}")
    }
}

fn cells_str(cells: &[&advent_of_code_common::grid::GridCell<GridCell>]) -> String {
    cells.iter().map(|c| c.data.c).collect()
}

fn rune_grid(input: &str) -> usize {
    let runes = extract_runes(input);
    let grid: Grid<GridCell> = clean_input(input).parse().expect("grid");
    grid.cells.iter().for_each(|(p, c)| {
        for rune in &runes {
            if rune.starts_with(c.data.c) {
                for dir in Direction::orthogonal() {
                    let sight_line = match dir {
                        Direction::North | Direction::South => {
                            grid.sight_line_n(p, &dir, rune.len())
                        }
                        Direction::East | Direction::West => {
                            grid.sight_line_wrapped(p, &dir, rune.len())
                        }
                        _ => unreachable!("Diagonal direction"),
                    };
                    if cells_str(&sight_line) == *rune {
                        for c in sight_line {
                            c.data.is_rune.set(true);
                        }
                    }
                }
            }
        }
    });
    grid.cells
        .iter()
        .filter(|(_, c)| c.data.is_rune.get())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        let input = "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
        assert_eq!(count_runes(input), 4);
        let input = read_to_string("../inputs/everybody/quest02part1.txt").expect("Input file");
        assert_eq!(count_runes(&input), 36);
    }

    #[test]
    fn part2() {
        let input = "WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ";
        assert_eq!(count_rune_symbols(input), 42);
        let input = read_to_string("../inputs/everybody/quest02part2.txt").expect("Input file");
        assert_eq!(count_rune_symbols(&input), 5225);
    }

    #[test]
    fn part3() {
        let input = "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL";
        assert_eq!(rune_grid(input), 10);
        let input = read_to_string("../inputs/everybody/quest02part3.txt").expect("Input file");
        assert_eq!(rune_grid(&input), 11305);
    }
}
