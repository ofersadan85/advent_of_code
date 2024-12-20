use advent_of_code_macros::aoc_tests;

fn parse_input(input: &str) -> Vec<usize> {
    let numbers: Vec<usize> = input
        .lines()
        .map(|line| line.trim().parse().unwrap_or(0))
        .collect();
    let mut elves: Vec<_> = numbers
        .split(|&x| x == 0)
        .map(|block| block.iter().sum())
        .collect();
    elves.sort_unstable();
    elves
}

fn part_1(elves: &[usize]) -> Option<usize> {
    elves.last().copied()
}

fn part_2(elves: &[usize]) -> usize {
    elves[(elves.len() - 3)..].iter().sum()
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "
    1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[test]
    fn example_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), Some(24000));
    }

    #[test]
    fn solution_1() {
        assert_eq!(part_1(&parse_input(&read_input())), Some(67633));
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE)), 45000);
    }

    #[test]
    fn solution_2() {
        assert_eq!(part_2(&parse_input(&read_input())), 199628);
    }
}
