use crate::{default_input_path, Solver};

fn parse_numbers(input: &str) -> (Vec<usize>, usize) {
    let mut max = 0;
    let numbers = input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .inspect(|n| {
            max = max.max(*n);
        })
        .collect();
    (numbers, max)
}

fn is_cutting(prev_line: &[usize], line: &[usize]) -> bool {
    let prev_start = prev_line[0].min(prev_line[1]);
    let prev_end = prev_line[0].max(prev_line[1]);
    let start = line[0].min(line[1]);
    let end = line[0].max(line[1]);
    let cross1 = prev_start < start && start < prev_end && prev_end < end;
    let cross2 = prev_start < end && end < prev_end && start < prev_start;
    cross1 || cross2 || (start == prev_start && end == prev_end)
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let (numbers, max) = parse_numbers(input);
        numbers
            .windows(2)
            .filter(|w| w[0].abs_diff(w[1]) == max / 2)
            .count()
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }

    fn file_chunk_separator(&self) -> &'static str {
        "\n"
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut result = 0;
        let (numbers, _) = parse_numbers(input);
        for (index, line) in numbers.windows(2).enumerate() {
            for prev_line in numbers.windows(2).take(index.saturating_sub(1)) {
                if is_cutting(prev_line, line) {
                    result += 1;
                }
            }
        }
        result
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }

    fn file_chunk_separator(&self) -> &'static str {
        "\n"
    }
}

struct Part3;
impl Solver<'_> for Part3 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut result = 0;
        let (numbers, max) = parse_numbers(input);
        for start in 1..=max {
            for end in 1..start {
                let new_line = vec![start, end];
                let cuts = numbers
                    .windows(2)
                    .filter(|prev_line| is_cutting(prev_line, &new_line))
                    .count();
                result = result.max(cuts);
            }
        }
        
        result
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }

    fn file_chunk_separator(&self) -> &'static str {
        "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, 4);
        expect_solution!(Part1, 1, 55);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 2, 21);
        expect_solution!(Part2, 3, 2924365);
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 4, 7);
        expect_solution!(Part3, 5, 2787);
    }
}
