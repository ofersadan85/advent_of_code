use advent_of_code_common::Solver;

fn parse_input(s: &str) -> Vec<usize> {
    s.split_ascii_whitespace()
        .filter_map(|line| line.parse().ok())
        .collect()
}

const fn fuel_per_mass(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

const fn fuel_per_mass_recursive(mass: usize) -> usize {
    let mut total_fuel = 0;
    let mut additional_fuel = fuel_per_mass(mass);
    while additional_fuel > 0 {
        total_fuel += additional_fuel;
        additional_fuel = fuel_per_mass(additional_fuel);
    }
    total_fuel
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_input(input).iter().map(|&m| fuel_per_mass(m)).sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        parse_input(input)
            .iter()
            .map(|&m| fuel_per_mass_recursive(m))
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 3154112);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 4728317);
    }
}
