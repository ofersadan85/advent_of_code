use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
enum ValueMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<isize> for ValueMode {
    type Error = isize;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
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
    AdjustRelativeBase = 9,
    Halt = 99,
}

impl Instruction {
    const fn parameter_count(&self) -> usize {
        match self {
            Self::Add | Self::Multiply | Self::LessThan | Self::Equals => 3,
            Self::Output | Self::AdjustRelativeBase => 1,
            Self::JumpIfTrue | Self::JumpIfFalse => 2,
            Self::Halt | Self::Input => 0, // Input is a special case
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
            9 => Ok(Self::AdjustRelativeBase),
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Running,
    AwaitingInput,
    Halted,
}

#[derive(Clone)]
pub struct IntcodeComputer {
    pub memory: HashMap<usize, isize>,
    pc: usize, // program counter
    queued_input: VecDeque<isize>,
    pub state: State,
    pub output: VecDeque<isize>,
    relative_base: isize,
}

impl FromStr for IntcodeComputer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Result<Vec<isize>, _> = s.trim().split(',').map(str::parse).collect();
        Ok(Self::new(memory?))
    }
}

impl IntcodeComputer {
    pub fn new(memory: Vec<isize>) -> Self {
        let memory: HashMap<usize, isize> = memory.into_iter().enumerate().collect();
        Self {
            memory,
            pc: 0,
            queued_input: VecDeque::new(),
            state: State::Running,
            output: VecDeque::new(),
            relative_base: 0,
        }
    }

    pub fn queue_input(&mut self, input: isize) {
        self.queued_input.push_back(input);
    }

    fn read_memory(&self, index: usize) -> isize {
        self.memory.get(&index).copied().unwrap_or(0)
    }

    fn write_index(&self, mode: &ValueMode, index: usize) -> usize {
        let value = self.read_memory(index);
        match mode {
            ValueMode::Position => Self::real_index(value),
            ValueMode::Relative => Self::real_index(self.relative_base + value),
            ValueMode::Immediate => panic!("Immediate mode is not valid for write parameters"),
        }
    }

    fn handle_input(&mut self, mode: &ValueMode) {
        let dest = self.write_index(mode, self.pc + 1);
        match self.queued_input.pop_front() {
            Some(value) => {
                self.set(value, dest);
                self.pc += 2; // Move past the input instruction and its parameter only when consumed
            }
            None => self.state = State::AwaitingInput,
        }
    }

    /// Small helper function to convert an isize index to usize, panicking if the index is negative.
    fn real_index(n: isize) -> usize {
        usize::try_from(n).expect("Index must be non-negative")
    }

    fn get(&self, mode: &ValueMode, index: usize) -> isize {
        let value = self.read_memory(index);
        match mode {
            ValueMode::Position => self.read_memory(Self::real_index(value)),
            ValueMode::Immediate => value,
            ValueMode::Relative => self.read_memory(Self::real_index(self.relative_base + value)),
        }
    }

    fn set(&mut self, value: isize, index: usize) {
        self.memory.insert(index, value);
    }

    pub fn run(&mut self) -> State {
        if matches!(self.state, State::AwaitingInput) {
            if self.queued_input.is_empty() {
                return State::AwaitingInput;
            }
            self.state = State::Running;
        }
        while self.state == State::Running {
            let opcode = Opcode::from(*self.memory.get(&self.pc).unwrap_or(&0));
            match opcode.instruction {
                Instruction::Add => {
                    let a = self.get(&opcode.modes[0], self.pc + 1);
                    let b = self.get(&opcode.modes[1], self.pc + 2);
                    let dest = self.write_index(&opcode.modes[2], self.pc + 3);
                    self.set(a + b, dest);
                }
                Instruction::Multiply => {
                    let a = self.get(&opcode.modes[0], self.pc + 1);
                    let b = self.get(&opcode.modes[1], self.pc + 2);
                    let dest = self.write_index(&opcode.modes[2], self.pc + 3);
                    self.set(a * b, dest);
                }
                Instruction::Input => {
                    self.handle_input(&opcode.modes[0]);
                    if self.state == State::AwaitingInput {
                        break;
                    }
                    continue; // Skip the normal pc increment at the end of the loop
                }
                Instruction::Output => {
                    let src = self.get(&opcode.modes[0], self.pc + 1);
                    self.output.push_back(src);
                }
                Instruction::JumpIfTrue | Instruction::JumpIfFalse => {
                    let a = self.get(&opcode.modes[0], self.pc + 1);
                    match (a != 0, &opcode.instruction) {
                        (true, Instruction::JumpIfTrue) | (false, Instruction::JumpIfFalse) => {
                            let b = self.get(&opcode.modes[1], self.pc + 2);
                            self.pc = Self::real_index(b);
                            continue; // Skip the normal pc increment at the end of the loop
                        }
                        _ => {}
                    }
                }
                Instruction::LessThan | Instruction::Equals => {
                    let a = self.get(&opcode.modes[0], self.pc + 1);
                    let b = self.get(&opcode.modes[1], self.pc + 2);
                    let dest = self.write_index(&opcode.modes[2], self.pc + 3);
                    self.set(
                        match (a.cmp(&b), &opcode.instruction) {
                            (Ordering::Less, Instruction::LessThan)
                            | (Ordering::Equal, Instruction::Equals) => 1,
                            _ => 0,
                        },
                        dest,
                    );
                }
                Instruction::AdjustRelativeBase => {
                    let a = self.get(&opcode.modes[0], self.pc + 1);
                    self.relative_base += a;
                }
                Instruction::Halt => {
                    self.state = State::Halted;
                    break;
                }
            }
            self.pc += 1 + opcode.instruction.parameter_count();
        }
        self.state
    }

    pub fn run_with_input(&mut self, input: impl Iterator<Item = isize>) -> State {
        self.queued_input.extend(input);
        self.run()
    }
}
