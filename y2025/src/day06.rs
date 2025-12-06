use advent_of_code_common::Solver;

fn transpose(s: &str) -> Vec<String> {
    let lines: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let max_len = lines.iter().map(Vec::len).max().unwrap_or(0);
    let mut result = vec![String::with_capacity(lines.len()); max_len];
    for line in lines {
        for (i, &ch) in line.iter().enumerate() {
            result[i].push(ch);
        }
    }
    result
}

fn quick_maths(numbers: &[usize], op: char) -> usize {
    match op {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => unreachable!("unknown operation"),
    }
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        let mut lines = input.lines();
        let mut result: Vec<(Vec<usize>, char)> = lines
            .next()
            .unwrap_or_default()
            .split_whitespace()
            .filter_map(|s| Some((vec![s.parse().ok()?], ' ')))
            .collect();
        for line in lines {
            for (i, n) in line.split_whitespace().enumerate() {
                if let Ok(n) = n.parse() {
                    result[i].0.push(n);
                } else {
                    result[i].1 = n.chars().next().unwrap_or(' ');
                }
            }
        }
        result
            .iter()
            .map(|(numbers, op)| quick_maths(numbers, *op))
            .sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        transpose(input)
            .chunk_by(|_, b| !b.trim().is_empty())
            .map(|chunk| {
                let mut current_op = "";
                let mut numbers = Vec::with_capacity(chunk.len());
                for line in chunk {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let n: usize = if line.ends_with('+') || line.ends_with('*') {
                        let (n, op) = line.split_at(line.len() - 1);
                        current_op = op;
                        n.trim().parse().unwrap_or(0)
                    } else {
                        line.trim().parse().unwrap_or(0)
                    };
                    numbers.push(n);
                }
                current_op
                    .chars()
                    .next()
                    .map_or(0, |op| quick_maths(&numbers, op))
            })
            .sum()
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
        expect_solution!(Part1, 0, 4277556);
        expect_solution!(Part1, 1, 4722948564882);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 3263827);
        expect_solution!(Part2, 1, 9581313737063);
    }
}
