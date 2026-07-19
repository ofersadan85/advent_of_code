use crate::intcode::IntcodeComputer;
use advent_of_code_macros::aoc_solver;
use std::iter::once;

#[aoc_solver(
    suffix = "example1",
    input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
    expected = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
)]
#[aoc_solver(
    suffix = "example2",
    input = "1102,34915192,34915192,7,4,7,99,0",
    expected = [1219070632396864]
)]
#[aoc_solver(
    suffix = "example3",
    input = "104,1125899906842624,99",
    expected = [1125899906842624]
)]
#[aoc_solver(file = "inputs/2019/day09.txt", expected = [3742852857])]
fn part_1(input: &str) -> Vec<isize> {
    let mut computer: IntcodeComputer = input.parse().expect("Invalid input");
    computer.run_with_input(once(1_isize));
    println!("Output: {:?}", computer.output);
    computer.output.drain(..).collect()
}

#[aoc_solver(file = "inputs/2019/day09.txt", expected = [73439])]
fn part_2(input: &str) -> Vec<isize> {
    let mut computer: IntcodeComputer = input.parse().expect("Invalid input");
    computer.run_with_input(once(2_isize));
    println!("Output: {:?}", computer.output);
    computer.output.drain(..).collect()
}
