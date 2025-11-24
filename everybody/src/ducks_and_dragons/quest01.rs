use crate::{default_input_path, Solver};

trait IndexMover {
    fn move_index(index: usize, dir: char, n: usize, len: usize) -> usize;
}

struct SimpleMover;
impl IndexMover for SimpleMover {
    fn move_index(index: usize, dir: char, n: usize, len: usize) -> usize {
        match dir {
            'L' => index.saturating_sub(n),
            'R' => index.saturating_add(n).min(len - 1),
            _ => index,
        }
    }
}

struct WrappingMover;
impl IndexMover for WrappingMover {
    fn move_index(index: usize, dir: char, n: usize, len: usize) -> usize {
        match dir {
            'L' => (index + len - (n % len)) % len,
            'R' => (index + n) % len,
            _ => index,
        }
    }
}

fn parse_names_moves(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let names: Vec<_> = lines.next().unwrap_or_default().trim().split(',').collect();
    let moves: Vec<_> = lines.nth(1).unwrap_or_default().trim().split(',').collect();
    (names, moves)
}

fn get_name_after_moves<M: IndexMover>(input: &str) -> &str {
    let (names, moves) = parse_names_moves(input);
    let mut index: usize = 0;
    for m in moves {
        let mut chars = m.chars();
        let dir = chars.next().unwrap_or_default();
        let n: usize = chars.as_str().parse().unwrap_or(0);
        index = M::move_index(index, dir, n, names.len());
    }
    names[index]
}

struct Part1;
impl<'a> Solver<'a> for Part1 {
    type Output = &'a str;

    fn solve(&self, input: &'a str) -> Self::Output {
        get_name_after_moves::<SimpleMover>(input)
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part2;
impl<'a> Solver<'a> for Part2 {
    type Output = &'a str;

    fn solve(&self, input: &'a str) -> Self::Output {
        get_name_after_moves::<WrappingMover>(input)
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

struct Part3;
impl<'a> Solver<'a> for Part3 {
    type Output = &'a str;

    fn solve(&self, input: &'a str) -> Self::Output {
        let (mut names, moves) = parse_names_moves(input);
        for m in moves {
            let mut chars = m.chars();
            let dir = chars.next().unwrap_or_default();
            let n: usize = chars.as_str().parse().unwrap_or(0);
            let new_index = WrappingMover::move_index(0, dir, n, names.len());
            names.swap(0, new_index);
        }
        names[0]
    }

    fn file_path(&self) -> std::path::PathBuf {
        default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expect_solution;
    use test_log::test;

    #[test]
    fn part_1() {
        expect_solution!(Part1, 0, "Fyrryn");
        expect_solution!(Part1, 2, "Axaliral");
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, "Elarzris");
        expect_solution!(Part2, 3, "Pylarquor");
    }

    #[test]
    fn part_3() {
        expect_solution!(Part3, 1, "Drakzyph");
        expect_solution!(Part3, 4, "Lirrilor");
    }
}
