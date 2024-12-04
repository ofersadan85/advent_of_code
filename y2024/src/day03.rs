use regex::Regex;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    re.captures_iter(input)
        .filter_map(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
        .collect()
}

fn parse_input_2(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let mut result = vec![];
    input
        .split("do()")
        .map(|s| s.split_once("don't()").map_or(s, |(s, _)| s))
        .for_each(|clean| {
            re.captures_iter(clean)
                .filter_map(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
                .for_each(|x| result.push(x));
        });
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_example_1() {
        let parsed = parse_input(EXAMPLE);
        assert_eq!(parsed, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
        assert_eq!(parsed.iter().map(|(a, b)| a * b).sum::<usize>(), 161);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2024/day03.txt").unwrap();
        let parsed = parse_input(&input);
        assert_eq!(parsed.iter().map(|(a, b)| a * b).sum::<usize>(), 161289189);
    }

    #[test]
    fn test_example_2() {
        let parsed = parse_input_2(EXAMPLE2);
        assert_eq!(parsed, vec![(2, 4), (8, 5)]);
        assert_eq!(parsed.iter().map(|(a, b)| a * b).sum::<usize>(), 48);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2024/day03.txt").unwrap();
        let parsed = parse_input_2(&input);
        assert_eq!(parsed.iter().map(|(a, b)| a * b).sum::<usize>(), 83595109);
    }
}
