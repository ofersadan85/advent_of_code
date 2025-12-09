use advent_of_code_common::Solver;

fn parse_input(s: &str) -> Vec<usize> {
    s.trim()
        .split(',')
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn run_program(memory: &mut [usize]) {
    let mut pc = 0; // program counter
    loop {
        match memory[pc] {
            1 => {
                let (src1, src2, dest) = (memory[pc + 1], memory[pc + 2], memory[pc + 3]);
                memory[dest] = memory[src1] + memory[src2];
                pc += 4;
            }
            2 => {
                let (src1, src2, dest) = (memory[pc + 1], memory[pc + 2], memory[pc + 3]);
                memory[dest] = memory[src1] * memory[src2];
                pc += 4;
            }
            99 => break,
            _ => panic!("Unknown opcode encountered"),
        }
    }
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut memory = parse_input(input);
        memory[1] = 12;
        memory[2] = 2;
        run_program(&mut memory);
        memory[0]
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let original_memory = parse_input(input);
        for noun in 0..100 {
            for verb in 0..100 {
                let mut memory = original_memory.clone();
                memory[1] = noun;
                memory[2] = verb;
                run_program(&mut memory);
                if memory[0] == 19_690_720 {
                    return 100 * noun + verb;
                }
            }
        }
        panic!("No valid noun and verb found");
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 6627023);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 4019);
    }
}
