use advent_of_code_common::file::split_lines_trim;
use std::collections::HashSet;

const PATH: &str = "inputs/day09.txt";
const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Movement {
    x: i32,
    y: i32,
}

type Point = Movement;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Stay,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
}

impl Direction {
    fn from_row(row: &str) -> Vec<Self> {
        let (dir, i) = row.trim().split_once(' ').unwrap();
        let direction = match dir {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => Self::Stay,
        };
        (0..i.parse().unwrap()).map(|_| direction).collect()
    }

    fn to_movement(self) -> Movement {
        match self {
            Self::Left => Movement { x: -1, y: 0 },
            Self::Right => Movement { x: 1, y: 0 },
            Self::Up => Movement { x: 0, y: 1 },
            Self::Down => Movement { x: 0, y: -1 },
            Self::Stay => Movement { x: 0, y: 0 },
            Self::UpLeft => Movement { x: -1, y: 1 },
            Self::DownLeft => Movement { x: -1, y: -1 },
            Self::UpRight => Movement { x: 1, y: 1 },
            Self::DownRight => Movement { x: 1, y: -1 },
        }
    }
}

fn input(example: bool, part_2: bool) -> Vec<Movement> {
    let data = if example {
        split_lines_trim(if part_2 { EXAMPLE2 } else { EXAMPLE })
    } else {
        split_lines_trim(&std::fs::read_to_string(PATH).unwrap())
    };
    data.iter()
        .flat_map(|row| Direction::from_row(row))
        .map(Direction::to_movement)
        .collect()
}

#[allow(clippy::enum_glob_use)]
fn knot_movement(previous: Point, current: &mut Point) {
    use Direction::*;
    let diff = Movement {
        x: previous.x - current.x,
        y: previous.y - current.y,
    };
    let movement = match diff {
        Movement { x: 2, y: 0 } => Right,
        Movement { x: -2, y: 0 } => Left,
        Movement { x: 0, y: 2 } => Up,
        Movement { x: 0, y: -2 } => Down,
        Movement { x: 2, y: 1 | 2 } | Movement { x: 1, y: 2 } => UpRight,
        Movement { x: -2, y: 1 | 2 } | Movement { x: -1, y: 2 } => UpLeft,
        Movement { x: 2, y: -1 | -2 } | Movement { x: 1, y: -2 } => DownRight,
        Movement { x: -2, y: -1 | -2 } | Movement { x: -1, y: -2 } => DownLeft,
        _ => Stay,
    }
    .to_movement();
    current.x += movement.x;
    current.y += movement.y;
}

fn rope(data: &[Movement], length: usize) -> usize {
    let mut tail_visited: HashSet<Point> = HashSet::new();
    let mut knots: Vec<Point> = (0..length).map(|_| Point { x: 0, y: 0 }).collect();
    for head_movement in data {
        knots[0].x += head_movement.x;
        knots[0].y += head_movement.y;
        let mut previous = knots[0];
        for knot in &mut knots {
            knot_movement(previous, knot);
            previous = *knot;
        }
        tail_visited.insert(*knots.last().unwrap());
    }
    tail_visited.len()
}

#[test]
fn example_1() {
    assert_eq!(rope(&input(true, false), 2), 13);
}

#[test]
fn task_1() {
    assert_eq!(rope(&input(false, false), 2), 6067);
}

#[test]
fn example_2() {
    assert_eq!(rope(&input(true, true), 10), 36);
}

#[test]
fn task_2() {
    assert_eq!(rope(&input(false, true), 10), 2471);
}
