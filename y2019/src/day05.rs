use crate::intcode::IntcodeComputer;
use advent_of_code_macros::aoc_solver;

#[aoc_solver(file = "inputs/2019/day05.txt", expected = 5182797)]
fn part_1(input: &str) -> isize {
    let output = IntcodeComputer::build_and_run(input, &[1]);
    output[..output.len() - 1]
        .iter()
        .enumerate()
        .for_each(|(i, &code)| assert_eq!(code, 0, "Expected diagnostic code 0 at position {i}"));
    *output.last().expect("Expected at least one output value")
}

#[aoc_solver(file = "inputs/2019/day05.txt", expected = 12077198)]
fn part_2(input: &str) -> isize {
    let output = IntcodeComputer::build_and_run(input, &[5]);
    *output.last().expect("Expected at least one output value")
}
