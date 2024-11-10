extern crate itertools;
use itertools::iproduct;

fn input(example: bool) -> Vec<usize> {
    const PATH: &str = "../inputs/2020/day01.txt";
    if example {
        "1721 979 366 299 675 1456".to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .split_ascii_whitespace()
    .map(|s| s.trim().parse().unwrap())
    .collect()
}

fn part_1(data: &[usize]) -> usize {
    let (a, b) = iproduct!(data, data)
        .find(|(&a, &b)| a + b == 2020)
        .unwrap();
    a * b
}

fn part_2(data: &[usize]) -> usize {
    let (a, b, c) = iproduct!(data, data, data)
        .find(|(&a, &b, &c)| a + b + c == 2020)
        .unwrap();
    a * b * c
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 514_579);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 542_619);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true)), 241_861_950);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 32_858_450);
}
