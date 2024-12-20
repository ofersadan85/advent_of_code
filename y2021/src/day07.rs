use advent_of_code_macros::aoc_tests;
use itertools::Itertools;

fn calc_fuel(data: &[u32]) -> u32 {
    let (min, max) = data.iter().minmax().into_option().unwrap_or((&0, &0));
    (*min..*max)
        .map(|i| data.iter().map(|x| x.abs_diff(i)).sum())
        .min()
        .unwrap_or_default()
}

fn calc_fuel_increasing(data: &[u32]) -> u32 {
    let (min, max) = data.iter().minmax().into_option().unwrap_or((&0, &0));
    (*min..*max)
        .map(|i| {
            data.iter()
                .map(|&x| {
                    let diff = x.abs_diff(i);
                    (diff * diff + diff) / 2
                })
                .sum()
        })
        .min()
        .unwrap_or_default()
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_1() {
        let data: Vec<_> = EXAMPLE.split(',').filter_map(|s| s.parse().ok()).collect();
        assert_eq!(calc_fuel(&data), 37);
    }

    #[test]
    fn example_2() {
        let data: Vec<_> = EXAMPLE.split(',').filter_map(|s| s.parse().ok()).collect();
        assert_eq!(calc_fuel_increasing(&data), 168);
    }

    #[test]
    fn part_1() {
        let data: Vec<_> = read_input()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        assert_eq!(calc_fuel(&data), 356_958);
    }

    #[test]
    fn part_2() {
        let data: Vec<_> = read_input()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        assert_eq!(calc_fuel_increasing(&data), 105_461_913);
    }
}
