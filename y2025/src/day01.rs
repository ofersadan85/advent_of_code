use advent_of_code_common::Solver;

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
    type Output = u32;

    fn solve(&self, input: &str) -> Self::Output {
        let mut count = 0;
        let mut acc = vec![50];
        for mut value in parse_input(input) {
            while value.abs() > 100 {
                let step = if value > 0 { 100 } else { -100 };
                acc.push(acc.last().expect("not empty") + step);
                value -= step;
            }
            acc.push(acc.last().expect("not empty") + value);
        }
        for window in acc.windows(2) {
            todo!()
        }
        count
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
        // expect_solution!(Part2, 0, 6);
        expect_solution!(Part2, 2, 2);
    }
}
