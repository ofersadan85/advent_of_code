use std::{cmp::Ordering, collections::VecDeque, str::FromStr};

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
            Self::Output => 1,
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
    pub memory: Vec<isize>,
    pc: usize, // program counter
    queued_input: VecDeque<isize>,
    pub state: State,
    pub output: VecDeque<isize>,
}

impl FromStr for IntcodeComputer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Result<Vec<isize>, _> = s.trim().split(',').map(str::parse).collect();
        Ok(Self::new(memory?))
    }
}

impl IntcodeComputer {
    pub const fn new(memory: Vec<isize>) -> Self {
        Self {
            memory,
            pc: 0,
            queued_input: VecDeque::new(),
            state: State::Running,
            output: VecDeque::new(),
        }
    }

    pub fn queue_input(&mut self, input: isize) {
        self.queued_input.push_back(input);
    }

    fn handle_input(&mut self) {
        let dest = usize::try_from(self.memory[self.pc + 1]).expect("Index must be non-negative");
        match self.queued_input.pop_front() {
            Some(value) => {
                self.memory[dest] = value;
                self.pc += 2; // Move past the input instruction and its parameter only when consumed
            }
            None => self.state = State::AwaitingInput,
        }
    }

    pub fn run(&mut self) -> State {
        if matches!(self.state, State::AwaitingInput) {
            if self.queued_input.is_empty() {
                return State::AwaitingInput;
            }
            self.state = State::Running;
        }
        let real_index =
            |n: isize| -> usize { usize::try_from(n).expect("Index must be non-negative") };
        let value = |mode: &ValueMode, mem: &[isize], index: usize| match mode {
            ValueMode::Position => mem[real_index(mem[index])],
            ValueMode::Immediate => mem[index],
        };
        while self.state == State::Running {
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
                    self.handle_input();
                    if self.state == State::AwaitingInput {
                        break;
                    }
                    continue; // Skip the normal pc increment at the end of the loop
                }
                Instruction::Output => {
                    let src = value(&opcode.modes[0], &self.memory, self.pc + 1);
                    self.output.push_back(src);
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
