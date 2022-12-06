use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

const PATH: &str = "inputs/day05.txt";
const EXAMPLE: &str = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
const EXAMPLE_OUTPUT_1: &str = "
.......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....";
const EXAMPLE_OUTPUT_2: &str = "
1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....";

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_points(&self) -> Vec<Point> {
        let mut result = vec![];
        let mut p = self.start;
        let x_inc = match self.start.x.cmp(&self.end.x) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        let y_inc = match self.start.y.cmp(&self.end.y) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        while (p.x, p.y) != (self.end.x, self.end.y) {
            result.push(p);
            p = Point {
                x: (p.x + x_inc),
                y: (p.y + y_inc),
            }
        }
        result.push(self.end);
        result
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

fn point_count(data: &[Line], diagonals: bool) -> HashMap<Point, i32> {
    let points: Vec<Point> = data
        .iter()
        .filter(|line| diagonals || !line.is_diagonal())
        .flat_map(Line::get_points)
        .collect();
    let mut counter = HashMap::new();
    for p in points {
        counter.entry(p).and_modify(|v| *v += 1).or_insert(1);
    }
    counter
}

fn setup_data(data: &[String]) -> Vec<Line> {
    data.iter()
        .map(|line| {
            let (a, b, c, d): (i32, i32, i32, i32) = line
                .replace(" -> ", ",")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Line {
                start: Point { x: a, y: b },
                end: Point { x: c, y: d },
            }
        })
        .collect()
}

#[allow(clippy::cast_sign_loss)] // This should be validated to be positive elsewhere
fn setup_output(points: &HashMap<Point, i32>, w: i32, h: i32) -> String {
    let mut board: Vec<Vec<String>> = (0..h)
        .map(|_| (0..w).map(|_| ".".to_string()).collect())
        .collect();
    for (p, num) in points {
        board[p.y as usize][p.x as usize] = num.to_string();
    }
    board.iter().map(|line| line.join("")).join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::file::split_lines_trim;

    #[test]
    fn example_1() {
        let lines = split_lines_trim(EXAMPLE);
        let data = setup_data(&lines);
        let counter = point_count(&data, false);
        let result = counter.values().filter(|v| **v >= 2).count();
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let lines = split_lines_trim(EXAMPLE);
        let data = setup_data(&lines);
        let counter = point_count(&data, true);
        let result = counter.values().filter(|v| **v >= 2).count();
        assert_eq!(result, 12);
    }

    #[test]
    fn task_1() {
        let lines = split_lines_trim(&std::fs::read_to_string(PATH).unwrap());
        let data = setup_data(&lines);
        let counter = point_count(&data, false);
        let result = counter.values().filter(|v| **v >= 2).count();
        assert_eq!(result, 6267);
    }

    #[test]
    fn task_2() {
        let lines = split_lines_trim(&std::fs::read_to_string(PATH).unwrap());
        let data = setup_data(&lines);
        let counter = point_count(&data, true);
        let result = counter.values().filter(|v| **v >= 2).count();
        assert_eq!(result, 20196);
    }
}
