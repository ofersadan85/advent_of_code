use itertools::Itertools;

fn is_possible_sum(numbers: &[usize], sum: usize) -> bool {
    numbers
        .iter()
        .combinations(2)
        .any(|c| c.into_iter().sum::<usize>() == sum)
}

fn is_valid_sequence(numbers: &[usize], preamble: usize) -> Option<usize> {
    for i in preamble..numbers.len() {
        if !is_possible_sum(&numbers[i - preamble..i], numbers[i]) {
            return Some(numbers[i]);
        }
    }
    None
}

fn weakness(numbers: &[usize], target: usize) -> Option<usize> {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let range = &numbers[i..j];
            if range.iter().sum::<usize>() == target {
                return Some(range.iter().min().unwrap() + range.iter().max().unwrap());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const EXAMPLE: &[usize] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_is_possible_sum() {
        let numbers = vec![35, 20, 15, 25, 47];
        assert!(is_possible_sum(&numbers, 40), "40 is possible");
        assert!(is_possible_sum(&numbers, 62), "62 is possible");
        assert!(!is_possible_sum(&numbers, 95), "95 is not possible");
    }

    #[test]
    fn test_is_valid_sequence() {
        assert_eq!(is_valid_sequence(EXAMPLE, 5), Some(127));
    }

    #[test]
    fn test_part1() {
        let numbers = read_to_string("../inputs/2020/day09.txt")
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(is_valid_sequence(&numbers, 25), Some(731031916));
    }

    #[test]
    fn test_weakness() {
        assert_eq!(weakness(EXAMPLE, 127), Some(62));
    }

    #[test]
    fn test_part2() {
        let numbers = read_to_string("../inputs/2020/day09.txt")
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(weakness(&numbers, 731031916), Some(93396727));
    }
}
