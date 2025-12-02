use advent_of_code_common::Solver;

struct WrapCountingInt {
    value: i32,
    count: usize,
}

impl std::ops::AddAssign<i32> for WrapCountingInt {
    fn add_assign(&mut self, rhs: i32) {
        self.value += rhs;
        while self.value > 100 {
            self.value -= 100;
            self.count += 1;
        }
        while self.value < 0 {
            self.value += 100;
            self.count += 1;
        }
        if self.value == 100 || self.value == 0 {
            self.value = 0;
            self.count += 1;
        }
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (dir, value) = line.trim().split_at(1);
            let value: i32 = value.parse().unwrap_or(0);
            match dir {
                "R" => value,
                "L" => -value,
                _ => unreachable!("unknown direction"),
            }
        })
        .collect()
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut count = 0;
        let mut n = 50;
        for value in parse_input(input) {
            n += value;
            n %= 100;
            count += usize::from(n == 0);
        }
        count
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}
struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let numbers = parse_input(input);
        let mut wrap_counter = WrapCountingInt {
            value: 50,
            count: 0,
        };
        let mut exact = false;
        for value in numbers {
            wrap_counter += value;
            if exact {
                exact = false;
                if value < 0 {
                    // Going negative after an exact wrap means we wrapped one too many times
                    wrap_counter.count -= 1;
                }
            }
            if wrap_counter.value == 0 {
                exact = true;
            }
        }
        wrap_counter.count
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;
    use test_log::test;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 3);
        expect_solution!(Part1, 1, 1040);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 6);
        expect_solution!(Part2, 1, 6027);
        expect_solution!(Part2, 2, 2);
    }
}
