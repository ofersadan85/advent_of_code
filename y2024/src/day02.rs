#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ListState {
    Increasing,
    Decreasing,
}

fn is_safe(values: &[usize]) -> bool {
    use ListState::{Decreasing, Increasing};
    let state = match (values.first(), values.last()) {
        (Some(&a), Some(&b)) if a < b => Increasing,
        (Some(&a), Some(&b)) if a > b => Decreasing,
        _ => return false,
    };
    !values.windows(2).any(|w| match &state {
        Increasing => w[0] >= w[1] || w[1] - w[0] > 3,
        Decreasing => w[1] >= w[0] || w[0] - w[1] > 3,
    })
}

fn is_safe_dampened(values: &[usize]) -> bool {
    if is_safe(values) {
        return true;
    }
    for i in 0..values.len() {
        let mut new_values = values.to_vec();
        new_values.remove(i);
        if is_safe(&new_values) {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE: &str = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

    #[test]
    fn test_example_1() {
        let values = parse_input(EXAMPLE);
        assert!(is_safe(&values[0]));
        assert!(!is_safe(&values[1]));
        assert!(!is_safe(&values[2]));
        assert!(!is_safe(&values[3]));
        assert!(!is_safe(&values[4]));
        assert!(is_safe(&values[5]));
        assert_eq!(values.iter().filter(|v| is_safe(&v)).count(), 2);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2024/day02.txt").unwrap();
        let values = parse_input(&input);
        assert_eq!(values.len(), 1000);
        assert_eq!(values.iter().filter(|v| is_safe(&v)).count(), 490);
    }

    #[test]
    fn test_example_2() {
        let values = parse_input(EXAMPLE);
        assert!(is_safe_dampened(&values[0]));
        assert!(!is_safe_dampened(&values[1]));
        assert!(!is_safe_dampened(&values[2]));
        assert!(is_safe_dampened(&values[3]));
        assert!(is_safe_dampened(&values[4]));
        assert!(is_safe_dampened(&values[5]));
        assert_eq!(values.iter().filter(|v| is_safe_dampened(&v)).count(), 4);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2024/day02.txt").unwrap();
        let values = parse_input(&input);
        assert_eq!(values.len(), 1000);
        assert_eq!(values.iter().filter(|v| is_safe_dampened(&v)).count(), 536);
    }
}
