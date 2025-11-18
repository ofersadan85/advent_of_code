use crate::{default_input_path, Solver};
use std::collections::BTreeSet;

fn parse_line(line: &str) -> Vec<usize> {
    line.trim()
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect()
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_line(input)
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .sum()
    }

    fn file_chunk_separator(&self) -> &'static str {
        "\n"
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &'_ str) -> Self::Output {
        parse_line(input)
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .take(20)
            .sum()
    }

    fn file_chunk_separator(&self) -> &'static str {
        "\n"
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}
struct Part3;
impl Solver<'_> for Part3 {
    type Output = usize;

    fn solve(&self, input: &'_ str) -> Self::Output {
        let mut numbers = parse_line(input);
        let mut count = 0;
        let mut removable = vec![];
        while !numbers.is_empty() {
            removable.clear();
            let set = numbers.iter().collect::<BTreeSet<_>>();
            for item in set {
                if let Some(pos) = numbers.iter().position(|x| x == item) {
                    removable.push(pos);
                }
            }
            removable.sort_unstable();
            for index in removable.iter().rev() {
                numbers.remove(*index);
            }
            count += 1;
        }
        count
    }

    fn file_chunk_separator(&self) -> &'static str {
        "\n"
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 29);
        expect_solution!(Part1, 1, 2383);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 2, 781);
        expect_solution!(Part2, 3, 260);
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 2, 3);
        expect_solution!(Part3, 4, 2413);
    }
}
