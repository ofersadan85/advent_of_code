use std::num::ParseIntError;

fn parse_input(s: &str) -> Result<Vec<usize>, ParseIntError> {
    s.trim().split(',').map(|s| s.parse::<usize>()).collect()
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

#[advent_of_code_macros::aoc_tests]
mod tests {
    #[test]
    fn part1() {
        let input = read_input();
        let mut memory = parse_input(&input).unwrap();
        memory[1] = 12;
        memory[2] = 2;
        run_program(&mut memory);
        assert_eq!(memory[0], 6627023);
    }

    #[test]
    fn part2() {
        let input = read_input();
        let original_memory = parse_input(&input).unwrap();
        for noun in 0..100 {
            for verb in 0..100 {
                let mut memory = original_memory.clone();
                memory[1] = noun;
                memory[2] = verb;
                run_program(&mut memory);
                if memory[0] == 19690720 {
                    assert_eq!(100 * noun + verb, 4019);
                    return;
                }
            }
        }
        panic!("No valid noun and verb found");
    }
}
