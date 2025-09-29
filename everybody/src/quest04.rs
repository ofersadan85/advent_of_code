fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn count_strikes(input: &str) -> usize {
    let values = parse_input(input);
    let min = values.iter().min().copied().unwrap_or_default();
    vec_abs_diff(&values, min)
}

fn vec_abs_diff(v: &[usize], d: usize) -> usize {
    v.iter().map(|x| x.abs_diff(d)).sum()
}

fn vec_best_diff(input: &str) -> usize {
    let mut values = parse_input(input);
    values.sort_unstable();
    vec_abs_diff(&values, values[values.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        assert_eq!(count_strikes("3 4 7 8"), 10);
        let input = read_to_string("../inputs/everybody/quest04part1.txt").expect("input file");
        assert_eq!(count_strikes(&input), 81);
    }

    #[test]
    fn part2() {
        let input = read_to_string("../inputs/everybody/quest04part2.txt").expect("input file");
        assert_eq!(count_strikes(&input), 953804);
    }

    #[test]
    fn part3() {
        assert_eq!(vec_best_diff("2 4 5 6 8"), 8);
        assert_eq!(vec_best_diff("2 10 10 10"), 8);
        let input = read_to_string("../inputs/everybody/quest04part3.txt").expect("input file");
        assert_eq!(vec_best_diff(&input), 126084734);
    }
}
