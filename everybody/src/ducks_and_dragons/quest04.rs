use crate::default_input_path;
use advent_of_code_common::Solver;

struct Gear {
    inner_teeth: f64,
    outer_teeth: f64,
}

impl std::str::FromStr for Gear {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inner, outer) = s.trim().split_once('|').unwrap_or((s, s));
        Ok(Self {
            inner_teeth: inner.trim().parse()?,
            outer_teeth: outer.trim().parse()?,
        })
    }
}

impl Gear {
    fn ratio(&self, other: &Self) -> f64 {
        self.outer_teeth / other.inner_teeth
    }
}

fn gear_chain_ratio(gears: &[Gear]) -> f64 {
    gears
        .windows(2)
        .fold(1.0, |ratio, w| ratio * w[0].ratio(&w[1]))
}

#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn last_gear_after_n_first_turns(gears: &[Gear], first: f64) -> usize {
    (gear_chain_ratio(gears) * first) as usize
}

fn parse_section(section: &str) -> Vec<Gear> {
    section
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect()
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let gears = parse_section(input);
        last_gear_after_n_first_turns(&gears, 2025.0)
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn solve(&self, input: &str) -> Self::Output {
        let gears = parse_section(input);
        (10_000_000_000_000.0 / gear_chain_ratio(&gears)).ceil() as usize
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part3;
impl Solver<'_> for Part3 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let gears = parse_section(input);
        last_gear_after_n_first_turns(&gears, 100.0)
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
        expect_solution!(Part1, 0, 32400);
        expect_solution!(Part1, 1, 15888);
        expect_solution!(Part1, 2, 20049);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 625_000_000_000);
        expect_solution!(Part2, 1, 1_274_509_803_922);
        expect_solution!(Part2, 3, 2_677_651_905_253);
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 4, 400);
        expect_solution!(Part3, 5, 6818);
        expect_solution!(Part3, 6, 103_502_736_822);
    }
}
