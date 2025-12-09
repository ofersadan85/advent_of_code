use advent_of_code_common::Solver;
use advent_of_code_common::grid::{Coords, Direction, Point};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    start: Point,
    end: Point,
    dir: Direction,
    length: isize,
}

impl Line {
    fn x_range(&self) -> std::ops::RangeInclusive<isize> {
        self.start.x.min(self.end.x)..=self.start.x.max(self.end.x)
    }

    fn y_range(&self) -> std::ops::RangeInclusive<isize> {
        self.start.y.min(self.end.y)..=self.start.y.max(self.end.y)
    }
}

fn parse_line(input: &str) -> Vec<Line> {
    let mut start = Point::default();
    input
        .split(',')
        .map(|segment| {
            let (dir_char, length_str) = segment.split_at(1);
            let dir = match dir_char {
                "U" => Direction::North,
                "D" => Direction::South,
                "L" => Direction::West,
                "R" => Direction::East,
                _ => panic!("Invalid direction"),
            };
            let length: isize = length_str.parse().unwrap_or(0);
            let line = Line {
                start,
                end: start + (dir * length),
                dir,
                length,
            };
            start = line.end;
            line
        })
        .collect()
}

fn parse_input(s: &str) -> (Vec<Line>, Vec<Line>) {
    let mut lines = s.lines();
    let a = parse_line(lines.next().unwrap_or_default());
    let b = parse_line(lines.next().unwrap_or_default());
    (a, b)
}

fn crossings(a: &[Line], b: &[Line]) -> HashSet<(Point, usize, usize)> {
    let mut results = HashSet::new();
    for (ai, a) in a.iter().enumerate() {
        for (bi, b) in b.iter().enumerate() {
            let ax = a.x_range();
            let ay = a.y_range();
            let bx = b.x_range();
            let by = b.y_range();
            let new_point = if ax.contains(bx.start()) && by.contains(ay.start()) {
                (*bx.start(), *ay.start()).as_point()
            } else if bx.contains(ax.start()) && ay.contains(by.start()) {
                (*ax.start(), *by.start()).as_point()
            } else {
                Point::default()
            };
            if new_point != Point::default() {
                results.insert((new_point, ai, bi));
            }
        }
    }
    results
}

struct Part1;
impl Solver<'_> for Part1 {
    type Output = isize;

    fn solve(&self, input: &str) -> Self::Output {
        let (a, b) = parse_input(input);
        crossings(&a, &b)
            .iter()
            .map(|(p, _, _)| p.x.abs() + p.y.abs())
            .min()
            .unwrap_or(0)
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

struct Part2;
impl Solver<'_> for Part2 {
    type Output = isize;

    fn solve(&self, input: &str) -> Self::Output {
        let (a, b) = parse_input(input);
        crossings(&a, &b)
            .iter()
            .map(|(p, ai, bi)| {
                let mut distance_a: isize = a[..*ai].iter().map(|line| line.length).sum();
                distance_a += p.manhattan_distance(&a[*ai].start);
                let mut distance_b: isize = b[..*bi].iter().map(|line| line.length).sum();
                distance_b += p.manhattan_distance(&b[*bi].start);
                distance_a + distance_b
            })
            .min()
            .unwrap_or(0)
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
        expect_solution!(Part1, 0, 6);
        expect_solution!(Part1, 1, 159);
        expect_solution!(Part1, 2, 135);
        expect_solution!(Part1, 3, 217);
    }

    #[test]
    fn part_2() {
        expect_solution!(Part2, 0, 30);
        expect_solution!(Part2, 1, 610);
        expect_solution!(Part2, 2, 410);
        expect_solution!(Part2, 3, 3454);
    }
}
