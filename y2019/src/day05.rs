use crate::intcode::IntcodeComputer;
use advent_of_code_macros::aoc_solver;
use std::iter::once;

#[aoc_solver(file = "inputs/2019/day05.txt", expected = 5182797)]
fn part_1(input: &str) -> isize {
    let mut computer: IntcodeComputer = input.parse().expect("Invalid input");
    computer.run_with_input(once(1_isize));
    computer
        .output
        .pop_back()
        .expect("Expected at least one output value")
}

#[aoc_solver(file = "inputs/2019/day05.txt", expected = 12077198)]
fn part_2(input: &str) -> isize {
    let mut computer: IntcodeComputer = input.parse().expect("Invalid input");
    computer.run_with_input(once(5_isize));
    computer
        .output
        .pop_back()
        .expect("Expected at least one output value")
}
