use advent_of_code_common::Solver;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    jolts: Vec<usize>,
}

impl std::str::FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let lights: Vec<bool> = split
            .next()
            .ok_or(())?
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .map(|c| c == '#')
            .collect();
        let buttons = split
            .take_while_ref(|s| s.starts_with('('))
            .map(|s| {
                s.trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .filter_map(|n| n.trim().parse::<usize>().ok())
                    .collect()
            })
            .collect();
        let jolts: Vec<usize> = split
            .next()
            .ok_or(())?
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        let machine = Machine {
            lights,
            buttons,
            jolts,
        };
        Ok(machine)
    }
}

impl Machine {
    fn min_clicks_lights(&self) -> Option<usize> {
        // BFS to find minimal clicks to reach lights state
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((vec![false; self.lights.len()], 0));
        visited.insert(queue[0].0.clone());

        while let Some((current_state, clicks)) = queue.pop_front() {
            if current_state == self.lights {
                return Some(clicks);
            }

            for button in &self.buttons {
                let mut next_state = current_state.clone();
                for &index in button {
                    next_state[index] = !next_state[index];
                }
                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, clicks + 1));
                }
            }
        }
        None
    }

    fn min_clicks_jolts(&self) -> Option<usize> {
        todo!()
    }
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        input
            .lines()
            .filter_map(|line| line.parse().ok())
            .filter_map(|machine: Machine| machine.min_clicks_lights())
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        input
            .lines()
            .filter_map(|line| line.parse().ok())
            .filter_map(|machine: Machine| machine.min_clicks_jolts())
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 7);
        expect_solution!(Part1, 1, 522);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 33);
        // expect_solution!(Part2, 1, 1562459680);
    }
}
