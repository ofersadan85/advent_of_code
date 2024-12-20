use advent_of_code_macros::aoc_tests;
use itertools::Itertools;

fn grow(v: &mut [usize], n: usize) {
    for _ in 0..n {
        v.rotate_left(1);
        v[6] += v[8];
    }
}

fn parse_input(data: &str) -> [usize; 9] {
    let mut result = [0; 9];
    let counter = data
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .counts();
    for (k, v) in counter {
        result[k] = v;
    }
    result
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn example_1() {
        let mut data = parse_input(EXAMPLE);
        grow(&mut data, 18);
        let result: usize = data.iter().sum();
        assert_eq!(result, 26);
    }

    #[test]
    fn example_2() {
        let mut data = parse_input(EXAMPLE);
        grow(&mut data, 256);
        let result: usize = data.iter().sum();
        assert_eq!(result, 26_984_457_539);
    }

    #[test]
    fn part_1() {
        let mut data = parse_input(&read_input());
        grow(&mut data, 80);
        let result: usize = data.iter().sum();
        assert_eq!(result, 361_169);
    }

    #[test]
    fn part_2() {
        let mut data = parse_input(&read_input());
        grow(&mut data, 256);
        let result: usize = data.iter().sum();
        assert_eq!(result, 1_634_946_868_992);
    }
}
