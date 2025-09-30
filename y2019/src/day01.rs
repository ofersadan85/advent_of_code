use std::num::ParseIntError;

fn parse_input(s: &str) -> Result<Vec<usize>, ParseIntError> {
    s.split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect()
}

fn fuel_per_mass(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn fuel_per_mass_recursive(mass: usize) -> usize {
    let mut total_fuel = 0;
    let mut additional_fuel = fuel_per_mass(mass);
    while additional_fuel > 0 {
        total_fuel += additional_fuel;
        additional_fuel = fuel_per_mass(additional_fuel);
    }
    total_fuel
}

#[advent_of_code_macros::aoc_tests]
mod tests {
    #[test]
    fn part1() {
        let input = read_input();
        let masses = parse_input(&input).unwrap();
        let total_fuel: usize = masses.iter().map(|&m| fuel_per_mass(m)).sum();
        assert_eq!(total_fuel, 3154112);
    }

    #[test]
    fn part2() {
        let input = read_input();
        let masses = parse_input(&input).unwrap();
        let total_fuel: usize = masses.iter().map(|&m| fuel_per_mass_recursive(m)).sum();
        assert_eq!(total_fuel, 4728317);
    }
}
