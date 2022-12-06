use advent_of_code_common::file::{lines_as_blocks, lines_as_numbers};

const PATH: &str = "inputs/day01.txt";
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

fn input(example: bool) -> Vec<usize> {
    let data = if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    };

    let mut elves: Vec<usize> = lines_as_blocks(&data)
        .iter()
        .map(|block| {
            lines_as_numbers(&block.join("\n"))
                .unwrap_or_default()
                .iter()
                .sum()
        })
        .collect();
    elves.sort_unstable();
    elves
}

fn part_1(elves: &[usize]) -> usize {
    elves.last().unwrap().to_owned()
}

fn part_2(elves: &[usize]) -> usize {
    elves[(elves.len() - 3)..].iter().sum()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 24000);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 67633);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 45000);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 199_628);
}
