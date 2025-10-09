use advent_of_code_macros::aoc_tests;
use std::{ops::Range, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv = 0, // Division => Register A / (2.pow(Combo Operand)) + Truncate to integer, then store in Register A
    Bxl = 1, // Bitwise XOR => Register B ^ Literal Operand + Store in Register B
    Bst = 2, // Combo Operand % 8 (keep only 3 bits) => Store in Register B
    Jnz = 3, // Jump if Register A is not zero => Instruction Pointer = Literal Operand
    Bxc = 4, // Bitwise XOR => Register B ^ Register C + Store in Register B (read operand but ignore it)
    Out = 5, // Combo Operand % 8 (keep only 3 bits) => Output (comma separated)
    Bdv = 6, // Like Adv (0) but store in Register B (Still read numerator from Register A)
    Cdv = 7, // Like Adv (0) but store in Register C (Still read numerator from Register A)
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    original_instructions: String,
    instructions: Vec<Instruction>,
    output: Vec<u8>,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a = lines
            .next()
            .unwrap_or_default()
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .parse()
            .map_err(|_| ())?;
        let b = lines
            .next()
            .unwrap_or_default()
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .parse()
            .map_err(|_| ())?;
        let c = lines
            .next()
            .unwrap_or_default()
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .parse()
            .map_err(|_| ())?;
        let original_instructions = lines
            .nth(1)
            .unwrap_or_default()
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .to_string();
        let instructions = original_instructions
            .split(',')
            .filter_map(|x| x.parse::<u8>().ok())
            .filter_map(|n| Instruction::try_from(n).ok())
            .collect();
        Ok(Self {
            a,
            b,
            c,
            ip: 0,
            original_instructions,
            instructions,
            output: Vec::new(),
        })
    }
}

impl Machine {
    fn combo_operator(&self) -> u64 {
        match self.instructions.get(self.ip + 1).expect("Invalid instruction pointer") {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => self.a, // 4
            Instruction::Out => self.b, // 5
            Instruction::Bdv => self.c, // 6
            Instruction::Cdv => unimplemented!("Invalid combo operator 7"),
        }
    }

    fn literal_operator(&self) -> u64 {
        *self.instructions.get(self.ip + 1).expect("Invalid instruction pointer") as u64
    }

    fn division(&self) -> u64 {
        // self.a / 2u64.pow(self.combo_operator() as u32)
        self.a >> self.combo_operator()
    }

    fn modulo(&self) -> u64 {
        // self.combo_operator() % 8
        self.combo_operator() & 7
    }

    const fn increment_instruction(&mut self) {
        self.ip += 2;
    }

    #[allow(clippy::cast_possible_truncation)]
    fn op(&mut self) -> bool {
        let instruction = self.instructions[self.ip];
        match instruction {
            Instruction::Adv => {
                self.a = self.division();
                self.increment_instruction();
                false
            }
            Instruction::Bxl => {
                self.b ^= self.literal_operator();
                self.increment_instruction();
                false
            }
            Instruction::Bst => {
                self.b = self.modulo();
                self.increment_instruction();
                false
            }
            Instruction::Jnz if self.a != 0 => {
                self.ip = self.literal_operator() as usize;
                false
            }
            Instruction::Jnz => {
                self.increment_instruction();
                false
            }
            Instruction::Bxc => {
                self.b ^= self.c;
                self.increment_instruction();
                false
            }
            Instruction::Out => {
                self.output.push(self.modulo() as u8);
                self.increment_instruction();
                true
            }
            Instruction::Bdv => {
                self.b = self.division();
                self.increment_instruction();
                false
            }
            Instruction::Cdv => {
                self.c = self.division();
                self.increment_instruction();
                false
            }
        }
    }

    fn run(&mut self) {
        while self.ip < self.instructions.len() {
            self.op();
        }
    }

    fn run_until_output(&mut self) {
        while self.ip < self.instructions.len() {
            if self.op() {
                break;
            }
        }
    }

    fn output(&self) -> String {
        self.output
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    fn reset_with(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
        self.output.clear();
        self.instructions = self
            .original_instructions
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .split(',')
            .filter_map(|x| x.parse::<u8>().ok())
            .filter_map(|n| Instruction::try_from(n).ok())
            .collect();
    }

    fn is_self_replicate(&mut self, current_run: u64) -> bool {
        self.reset_with(current_run);
        loop {
            self.run_until_output();
            if !self.original_instructions.starts_with(&self.output()) {
                return false;
            } else if self.ip >= self.instructions.len() {
                break;
            }
        }
        self.output() == self.original_instructions
    }
}

fn find_replicator_recursive(machine: &Machine, range: Range<u64>, previous: &str) -> Option<u64> {
    let mut current_machine = machine.clone();
    for n in range {
        current_machine.reset_with(n);
        current_machine.run_until_output();
        let output = if previous.is_empty() {
            current_machine.output()
        } else {
            [current_machine.output(), previous.to_string()].join(",")
        };
        if output == current_machine.original_instructions {
            return Some(n);
        } else if current_machine.original_instructions.ends_with(&output) {
            let new_range = n * 8..(n + 1) * 8;
            let inner = find_replicator_recursive(machine, new_range, &output);
            if inner.is_some() {
                return inner;
            }
        }
    }
    None
}

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "Register A: 729
                            Register B: 0
                            Register C: 0

                            Program: 0,1,5,4,3,0";
    const EXAMPLE2: &str = "Register A: 2024
                            Register B: 0
                            Register C: 0

                            Program: 0,3,5,4,3,0";

    #[test]
    fn example_1() {
        let mut machine: Machine = EXAMPLE1.parse().unwrap();
        machine.run();
        assert_eq!(machine.output(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_1() {
        let mut machine: Machine = read_input().parse().unwrap();
        machine.run();
        assert_eq!(machine.output(), "2,1,3,0,5,2,3,7,1");
    }

    #[test]
    fn example_2() {
        let machine: Machine = EXAMPLE2.parse().unwrap();
        assert_eq!(find_replicator_recursive(&machine, 1..8, ""), Some(117440),);
    }

    #[test]
    fn part_2() {
        let machine: Machine = read_input().parse().unwrap();
        assert_eq!(
            find_replicator_recursive(&machine, 1..8, ""),
            Some(107416732707226),
        );
    }
}
