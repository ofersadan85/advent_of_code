use advent_of_code_common::file::split_lines;
const PATH: &str = "../inputs/2021/day02.txt";
const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

enum Direction {
    Forward,
    Up,
    Down,
}

fn navigate(data: &[(Direction, i32)]) -> i32 {
    let mut depth = 0;
    let mut forward = 0;
    for (direction, value) in data {
        match direction {
            Direction::Forward => forward += value,
            Direction::Down => depth += value,
            Direction::Up => depth -= value,
        }
    }
    depth * forward
}

fn navigate_aim(data: &[(Direction, i32)]) -> i32 {
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    for (direction, value) in data {
        match direction {
            Direction::Forward => {
                forward += value;
                depth += aim * value;
            }
            Direction::Down => aim += value,
            Direction::Up => aim -= value,
        }
    }
    depth * forward
}

fn input(example: bool) -> Vec<(Direction, i32)> {
    if example {
        split_lines(EXAMPLE)
    } else {
        split_lines(&std::fs::read_to_string(PATH).unwrap())
    }
    .iter()
    .map(|row| {
        let mut split = row.split_ascii_whitespace();
        let direction = split.next().unwrap();
        let value = split.next().unwrap().parse().unwrap_or(0);
        let dir = match direction {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!(),
        };
        (dir, value)
    })
    .collect()
}

#[test]
fn example_1() {
    assert_eq!(navigate(&input(true)), 150);
}

#[test]
fn task_1() {
    assert_eq!(navigate(&input(false)), 1_855_814);
}

#[test]
fn task_2() {
    assert_eq!(navigate_aim(&input(false)), 1_845_455_714);
}
