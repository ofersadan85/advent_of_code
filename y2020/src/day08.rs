use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let op = parts.next().ok_or_else(|| "Missing operation")?;
        let arg = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "Invalid argument")?;
        match op {
            "nop" => Ok(Self::Nop(arg)),
            "acc" => Ok(Self::Acc(arg)),
            "jmp" => Ok(Self::Jmp(arg)),
            _ => Err("Unknown instruction"),
        }
    }
}

fn parse_input(s: &str) -> Result<Vec<Instruction>> {
    Ok(s.lines()
        .filter_map(|l| l.trim().parse::<Instruction>().ok())
        .collect())
}

fn execute(instructions: &[Instruction]) -> (i32, bool) {
    let mut acc = 0;
    let mut pc = 0;
    let mut visited = vec![false; instructions.len()];
    while pc < instructions.len() && !visited[pc] {
        visited[pc] = true;
        match instructions[pc] {
            Instruction::Nop(_) => pc += 1,
            Instruction::Acc(arg) => {
                acc += arg;
                pc += 1;
            }
            Instruction::Jmp(arg) => pc = (pc as i32 + arg) as usize,
        }
    }
    (acc, pc == instructions.len())
}

fn fix_bug(instructions: &[Instruction]) -> i32 {
    for (i, instruction) in instructions.iter().enumerate() {
        let mut modified = instructions.to_vec();
        match instruction {
            Instruction::Nop(arg) => modified[i] = Instruction::Jmp(*arg),
            Instruction::Jmp(arg) => modified[i] = Instruction::Nop(*arg),
            _ => continue,
        }
        let (acc, terminated) = execute(&modified);
        if terminated {
            return acc;
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = " nop +0
                            acc +1
                            jmp +4
                            acc +3
                            jmp -3
                            acc -99
                            acc +1
                            jmp -4
                            acc +6";
    #[test]
    fn test_execute() {
        let instructions = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];

        let parsed = parse_input(EXAMPLE).unwrap();
        assert_eq!(parsed, instructions);
        assert_eq!(execute(&instructions), (5, false));
    }

    #[test]
    fn part1() {
        let input = include_str!("day08.txt");
        let instructions = parse_input(input).unwrap();
        assert_eq!(execute(&instructions).0, 1337);
    }

    #[test]
    fn part2() {
        let input = include_str!("day08.txt");
        let instructions = parse_input(input).unwrap();
        assert_eq!(fix_bug(&instructions), 1358);
    }
}
