use crate::intcode::IntcodeComputer;
use advent_of_code_macros::aoc_solver;

#[aoc_solver(file = "inputs/2019/day02.txt", expected = 6627023)]
fn part_1(input: &str) -> isize {
    let mut computer: IntcodeComputer = input.parse().expect("Invalid input");
    computer.memory.insert(1, 12);
    computer.memory.insert(2, 2);
    computer.run();
    *computer.memory.get(&0).unwrap_or(&0)
}

#[aoc_solver(file = "inputs/2019/day02.txt", expected = 4019)]
fn part_2(input: &str) -> isize {
    let original: IntcodeComputer = input.parse().expect("Invalid input");
    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = original.clone();
            computer.memory.insert(1, noun);
            computer.memory.insert(2, verb);
            computer.run();
            if *computer.memory.get(&0).unwrap_or(&0) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No valid noun and verb found");
}
