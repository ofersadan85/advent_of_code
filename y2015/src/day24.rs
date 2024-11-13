use itertools::Itertools;

fn best_balance(input: &[usize], compartments: usize) -> usize {
    let total = input.iter().sum::<usize>();
    let mut center_options: Vec<Vec<usize>> = vec![];
    for k in 1..=input.len() {
        center_options.extend(input.iter().copied().combinations(k).filter(|c| {
            let this_sum = c.iter().copied().sum::<usize>();
            this_sum == total / compartments && (total - this_sum) % (compartments - 1) == 0
        }));
        if !center_options.is_empty() {
            break;
        }
    }
    center_options
        .iter()
        .map(|c| c.iter().product::<usize>())
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_example_1() {
        let input: Vec<_> = (1..=5).chain(7..=11).collect();
        let result = best_balance(&input, 3);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_part_1() {
        let input: Vec<_> = read_to_string("../inputs/2015/day24.txt")
            .unwrap()
            .lines()
            .filter_map(|s| s.parse().ok())
            .collect();
        let result = best_balance(&input, 3);
        assert_eq!(result, 11846773891);
    }

    #[test]
    fn test_example_2() {
        let input: Vec<_> = (1..=5).chain(7..=11).collect();
        let result = best_balance(&input, 4);
        assert_eq!(result, 44);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<_> = read_to_string("../inputs/2015/day24.txt")
            .unwrap()
            .lines()
            .filter_map(|s| s.parse().ok())
            .collect();
        let result = best_balance(&input, 4);
        assert_eq!(result, 80393059);
    }
}
