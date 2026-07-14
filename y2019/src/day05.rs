use std::cmp::Ordering;

use advent_of_code_macros::aoc_solver;

#[derive(Debug, PartialEq, Eq)]
enum ValueMode {
    Position,
    Immediate,
}

impl TryFrom<isize> for ValueMode {
    type Error = isize;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl Instruction {
    const fn parameter_count(&self) -> usize {
        match self {
            Self::Add | Self::Multiply | Self::LessThan | Self::Equals => 3,
            Self::Input | Self::Output => 1,
            Self::JumpIfTrue | Self::JumpIfFalse => 2,
            Self::Halt => 0,
        }
    }
}

impl TryFrom<isize> for Instruction {
    type Error = isize;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value % 100 {
            1 => Ok(Self::Add),
            2 => Ok(Self::Multiply),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            99 => Ok(Self::Halt),
            _ => Err(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Opcode {
    instruction: Instruction,
    modes: [ValueMode; 3],
}

impl From<isize> for Opcode {
    fn from(value: isize) -> Self {
        let instruction = Instruction::try_from(value).expect("Invalid instruction");
        let modes = [
            ValueMode::try_from((value / 100) % 10).expect("Invalid mode for parameter 1"),
            ValueMode::try_from((value / 1000) % 10).expect("Invalid mode for parameter 2"),
            ValueMode::try_from((value / 10000) % 10).expect("Invalid mode for parameter 3"),
        ];
        Self { instruction, modes }
    }
}

struct IntcodeComputer {
    memory: Vec<isize>,
    pc: usize, // program counter
}

impl IntcodeComputer {
    const fn new(memory: Vec<isize>) -> Self {
        Self { memory, pc: 0 }
    }

    fn run(&mut self, input_values: &[isize]) -> Vec<isize> {
        let mut input_values_iter = input_values.iter();
        let mut output = Vec::new();
        let real_index =
            |n: isize| -> usize { usize::try_from(n).expect("Index must be non-negative") };
        let value = |mode: &ValueMode, mem: &[isize], index: usize| match mode {
            ValueMode::Position => mem[real_index(mem[index])],
            ValueMode::Immediate => mem[index],
        };

        loop {
            let opcode = Opcode::from(self.memory[self.pc]);
            match opcode.instruction {
                Instruction::Add => {
                    let a = value(&opcode.modes[0], &self.memory, self.pc + 1);
                    let b = value(&opcode.modes[1], &self.memory, self.pc + 2);
                    let dest = real_index(self.memory[self.pc + 3]);
                    self.memory[dest] = a + b;
                }
                Instruction::Multiply => {
                    let a = value(&opcode.modes[0], &self.memory, self.pc + 1);
                    let b = value(&opcode.modes[1], &self.memory, self.pc + 2);
                    let dest = real_index(self.memory[self.pc + 3]);
                    self.memory[dest] = a * b;
                }
                Instruction::Input => {
                    let dest = real_index(self.memory[self.pc + 1]);
                    println!("Reading input and storing in position {dest}");
                    self.memory[dest] = *input_values_iter.next().expect("Expected input value");
                }
                Instruction::Output => {
                    let src = value(&opcode.modes[0], &self.memory, self.pc + 1);
                    output.push(src);
                    println!("Output: {src}");
                }
                Instruction::JumpIfTrue | Instruction::JumpIfFalse => {
                    let a = value(&opcode.modes[0], &self.memory, self.pc + 1);
                    match (a != 0, &opcode.instruction) {
                        (true, Instruction::JumpIfTrue) | (false, Instruction::JumpIfFalse) => {
                            let b = value(&opcode.modes[1], &self.memory, self.pc + 2);
                            self.pc = real_index(b);
                            continue; // Skip the normal pc increment at the end of the loop
                        }
                        _ => {}
                    }
                }
                Instruction::LessThan | Instruction::Equals => {
                    let a = value(&opcode.modes[0], &self.memory, self.pc + 1);
                    let b = value(&opcode.modes[1], &self.memory, self.pc + 2);
                    let dest = real_index(self.memory[self.pc + 3]);
                    self.memory[dest] = match (a.cmp(&b), &opcode.instruction) {
                        (Ordering::Less, Instruction::LessThan)
                        | (Ordering::Equal, Instruction::Equals) => 1,
                        _ => 0,
                    };
                }
                Instruction::Halt => break,
            }
            self.pc += 1 + opcode.instruction.parameter_count();
        }
        output
    }
}

fn with_inputs(input: &str, machine_input: &[isize]) -> isize {
    let memory: Vec<isize> = input
        .trim()
        .split(',')
        .map(|line| line.parse().expect("valid number input"))
        .collect();
    let mut computer = IntcodeComputer::new(memory);
    let output = computer.run(machine_input);
    output[..output.len() - 1]
        .iter()
        .enumerate()
        .for_each(|(i, &code)| assert_eq!(code, 0, "Expected diagnostic code 0 at position {i}"));
    *output.last().expect("Expected at least one output value")
}

#[aoc_solver(file = "inputs/2019/day05.txt", expected = 5182797)]
fn part_1(input: &str) -> isize {
    with_inputs(input, &[1])
}

#[aoc_solver(file = "inputs/2019/day05.txt", expected = 12077198)]
fn part_2(input: &str) -> isize {
    with_inputs(input, &[5])
}
