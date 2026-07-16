use crate::intcode::{IntcodeComputer, State};
use advent_of_code_macros::aoc_solver;
use itertools::Itertools;

fn run_amplifiers(program: &[isize], phases: &[isize]) -> isize {
    let mut input = 0;
    let mut computers = vec![IntcodeComputer::new(program.to_vec()); 5];
    for (i, &phase) in phases.iter().enumerate() {
        computers[i].run_with_input([phase, input].into_iter());
        assert_eq!(
            computers[i].state,
            State::Halted,
            "Computer should be halted after execution"
        );
        assert_eq!(computers[i].output.len(), 1);
        input = computers[i].output[0];
    }
    input
}

fn run_amplifiers_with_feedback(program: &[isize], phases: &[isize]) -> isize {
    let mut computers = vec![IntcodeComputer::new(program.to_vec()); 5];
    for (i, &phase) in phases.iter().enumerate() {
        computers[i].queue_input(phase);
    }
    let mut input = 0;
    loop {
        let mut all_halted = true;
        for computer in &mut computers {
            computer.queue_input(input);
            computer.run();
            if computer.state != State::Halted {
                all_halted = false;
            }
            if let Some(output) = computer.output.pop_front() {
                input = output;
            }
        }
        if all_halted {
            break;
        }
    }
    input
}

const EXAMPLE1: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
const EXAMPLE2: &str = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
const EXAMPLE3: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

#[aoc_solver(
    suffix = "example1",
    input = EXAMPLE1,
    expected = 43210
)]
#[aoc_solver(
    suffix = "example2",
    input = EXAMPLE2,
    expected = 54321
)]
#[aoc_solver(
    suffix = "example3",
    input = EXAMPLE3,
    expected = 65210
)]
#[aoc_solver(file = "inputs/2019/day07.txt", expected = 20413)]
fn part_1(program: &str) -> isize {
    let program: Vec<isize> = program
        .trim()
        .split(',')
        .map(|s| s.parse().expect("to parse an integer"))
        .collect();
    (0..5)
        .permutations(5)
        .map(|phases| run_amplifiers(&program, &phases))
        .max()
        .expect("has a max")
}

const EXAMPLE4: &str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
const EXAMPLE5: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

#[aoc_solver(
    suffix = "example1",
    input = EXAMPLE4,
    expected = 139629729
)]
#[aoc_solver(
    suffix = "example2",
    input = EXAMPLE5,
    expected = 18216
)]
#[aoc_solver(file = "inputs/2019/day07.txt", expected = 3321777)]
fn part_2(program: &str) -> isize {
    let program: Vec<isize> = program
        .trim()
        .split(',')
        .map(|s| s.parse().expect("to parse an integer"))
        .collect();
    (5..10)
        .permutations(5)
        .map(|phases| run_amplifiers_with_feedback(&program, &phases))
        .max()
        .expect("has a max")
}
