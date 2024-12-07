fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (target, rest) = line.split_once(": ").unwrap_or_default();
            let target = target.trim().parse().unwrap_or_default();
            let values = rest
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            (target, values)
        })
        .collect()
}

fn next_step_1(values: &[usize], next: usize, target: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(values.len() * 2);
    result.extend(values.iter().map(|&v| v + next).filter(|v| *v <= target));
    result.extend(values.iter().map(|&v| v * next).filter(|v| *v <= target));
    result
}

fn is_valid_line_1(values: &[usize], target: usize) -> bool {
    let mut processed = vec![values[0]];
    for value in values.iter().skip(1) {
        processed = next_step_1(&processed, *value, target);
        if processed.is_empty() {
            return false;
        }
    }
    processed.contains(&target)
}

fn next_step_2(values: &[usize], next: usize, target: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(values.len() * 2);
    result.extend(values.iter().map(|&v| v + next).filter(|v| *v <= target));
    result.extend(values.iter().map(|&v| v * next).filter(|v| *v <= target));
    result.extend(
        values
            .iter()
            .filter_map(|&v| format!("{v}{next}").parse().ok())
            .filter(|v: &usize| *v <= target),
    );
    result
}

fn is_valid_line_2(values: &[usize], target: usize) -> bool {
    let mut processed = vec![values[0]];
    for value in values.iter().skip(1) {
        processed = next_step_2(&processed, *value, target);
        if processed.is_empty() {
            return false;
        }
    }
    processed.contains(&target)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const EXAMPLE: &str = "190: 10 19
                        3267: 81 40 27
                        83: 17 5
                        156: 15 6
                        7290: 6 8 6 15
                        161011: 16 10 13
                        192: 17 8 14
                        21037: 9 7 18 13
                        292: 11 6 16 20";

    #[test]
    fn example_1() {
        let lines = parse_input(EXAMPLE);
        let result: usize = lines
            .iter()
            .filter(|(target, values)| is_valid_line_1(values, *target))
            .map(|(t, _)| t)
            .sum();
        assert_eq!(result, 3749);
    }

    #[test]
    fn part_1() {
        let input = read_to_string("../inputs/2024/day07.txt").unwrap();
        let lines = parse_input(&input);
        let result: usize = lines
            .iter()
            .filter(|(target, values)| is_valid_line_1(values, *target))
            .map(|(t, _)| t)
            .sum();
        assert_eq!(result, 7885693428401);
    }
    #[test]
    fn example_2() {
        let lines = parse_input(EXAMPLE);
        let result: usize = lines
            .iter()
            .filter(|(target, values)| is_valid_line_2(values, *target))
            .map(|(t, _)| t)
            .sum();
        assert_eq!(result, 11387);
    }

    #[test]
    fn part_2() {
        let input = read_to_string("../inputs/2024/day07.txt").unwrap();
        let lines = parse_input(&input);
        let result: usize = lines
            .iter()
            .filter(|(target, values)| is_valid_line_2(values, *target))
            .map(|(t, _)| t)
            .sum();
        assert_eq!(result, 348360680516005);
    }
}
