use advent_of_code_macros::aoc_tests;

fn parse_input(input: &str) -> Vec<(usize, usize, usize, usize)> {
    input
        .lines()
        .filter_map(|row| {
            let (range1, range2) = row.trim().split_once(',')?;
            let (min1, max1) = range1.split_once('-')?;
            let (min2, max2) = range2.split_once('-')?;
            Some((
                min1.parse().ok()?,
                max1.parse().ok()?,
                min2.parse().ok()?,
                max2.parse().ok()?,
            ))
        })
        .collect()
}

fn part_1(data: &[(usize, usize, usize, usize)]) -> usize {
    data.iter()
        .filter(|(min1, max1, min2, max2)| {
            (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
        })
        .count()
}
fn part_2(data: &[(usize, usize, usize, usize)]) -> usize {
    data.iter()
        .filter(|(min1, max1, min2, max2)| {
            (min1 <= min2 && max1 >= max2)
                || (min2 <= min1 && max2 >= max1)
                || (max1 >= min2 && max1 <= max2)
                || (min1 >= min2 && min1 <= max2)
        })
        .count()
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn example_1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn solution_1() {
        let input = parse_input(&read_input());
        assert_eq!(part_1(&input), 513);
    }

    #[test]
    fn example_2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part_2(&input), 4);
    }

    #[test]
    fn solution_2() {
        let input = parse_input(&read_input());
        assert_eq!(part_2(&input), 878);
    }
}
