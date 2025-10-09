use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
    #[default]
    UnknownInstruction,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let instruction = split
            .next()
            .ok_or_else(|| anyhow!("No instruction found"))?;
        let second = split
            .next()
            .ok_or_else(|| anyhow!("No second part"))?
            .trim_start_matches('+')
            .trim_end_matches(',');
        let third: i32 = split
            .next()
            .unwrap_or("")
            .trim_start_matches('+')
            .parse()
            .unwrap_or_default();

        let instruction = match (instruction, second, third) {
            ("hlf", "a", _) => Self::Half(Register::A),
            ("hlf", "b", _) => Self::Half(Register::B),
            ("tpl", "a", _) => Self::Triple(Register::A),
            ("tpl", "b", _) => Self::Triple(Register::B),
            ("inc", "a", _) => Self::Increment(Register::A),
            ("inc", "b", _) => Self::Increment(Register::B),
            ("jmp", d, _) => Self::Jump(d.parse()?),
            ("jie", "a", d) => Self::JumpIfEven(Register::A, d),
            ("jie", "b", d) => Self::JumpIfEven(Register::B, d),
            ("jio", "a", d) => Self::JumpIfOne(Register::A, d),
            ("jio", "b", d) => Self::JumpIfOne(Register::B, d),
            _ => Self::UnknownInstruction,
        };
        Ok(instruction)
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn process(instructions: &[Instruction], mut a: u32, mut b: u32) -> (u32, u32) {
    let mut index = 0;
    while index < instructions.len() {
        let current = instructions
            .get(index)
            .expect("Inside the instruction bounds");
        let mut new_index = index as i32 + 1;
        (a, b, new_index) = match current {
            Instruction::Half(Register::A) => (a / 2, b, new_index),
            Instruction::Half(Register::B) => (a, b / 2, new_index),
            Instruction::Triple(Register::A) => (a * 3, b, new_index),
            Instruction::Triple(Register::B) => (a, b * 3, new_index),
            Instruction::Increment(Register::A) => (a + 1, b, new_index),
            Instruction::Increment(Register::B) => (a, b + 1, new_index),
            Instruction::JumpIfEven(Register::A, d) if a.is_multiple_of(2) => {
                (a, b, index as i32 + d)
            }
            Instruction::JumpIfEven(Register::B, d) if b.is_multiple_of(2) => {
                (a, b, index as i32 + d)
            }
            Instruction::JumpIfOne(Register::A, d) if a == 1 => (a, b, index as i32 + d),
            Instruction::JumpIfOne(Register::B, d) if b == 1 => (a, b, index as i32 + d),
            Instruction::JumpIfOne(_, _) | Instruction::JumpIfEven(_, _) => (a, b, new_index),
            Instruction::Jump(d) => (a, b, index as i32 + d),
            Instruction::UnknownInstruction => break,
        };
        if new_index >= 0 && new_index < instructions.len() as i32 {
            index = new_index as usize;
        } else {
            break;
        }
    }
    (a, b)
}

fn parse_input(s: &str) -> Vec<Instruction> {
    s.lines().filter_map(|s| s.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day23.txt").unwrap();
        let (a, b) = process(&parse_input(&input), 0, 0);
        assert_eq!(a, 1, "a");
        assert_eq!(b, 170, "b");
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day23.txt").unwrap();
        let (a, b) = process(&parse_input(&input), 1, 0);
        assert_eq!(a, 1, "a");
        assert_eq!(b, 247, "b");
    }
}
