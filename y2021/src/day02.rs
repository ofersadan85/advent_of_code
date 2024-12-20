use advent_of_code_macros::aoc_tests;

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

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|row| {
            let mut split = row.split_whitespace();
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

#[aoc_tests]
mod tests {
    const EXAMPLE1: &str = "forward 5
                            down 5
                            forward 8
                            up 3
                            down 8
                            forward 2";

    #[test]
    fn example_1() {
        let data = parse_input(EXAMPLE1);
        assert_eq!(navigate(&data), 150);
    }

    #[test]
    fn part_1() {
        let data = parse_input(&read_input());
        assert_eq!(navigate(&data), 1_855_814);
    }

    #[test]
    fn part_2() {
        let data = parse_input(&read_input());
        assert_eq!(navigate_aim(&data), 1_845_455_714);
    }
}
