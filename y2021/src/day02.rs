enum Direction {
    Forward,
    Up,
    Down,
}

fn navigate(data: Vec<(Direction, i32)>) -> i32 {
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

fn navigate_aim(data: Vec<(Direction, i32)>) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::{file::get_data, split_lines};

    fn setup_data(data: &[String]) -> Vec<(Direction, i32)> {
        data.iter()
            .map(|line| {
                let mut split = line.split_ascii_whitespace();
                let direction = split.next().unwrap();
                let value = split.next().unwrap().parse().unwrap_or(0);
                let dir = match direction {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => panic!("Unexpected Value"),
                };
                (dir, value)
            })
            .collect()
    }

    #[test]
    fn example() {
        let data = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        let data = setup_data(&split_lines(data));
        let result = navigate(data);
        assert_eq!(result, 150);
    }

    #[test]
    fn task_1() {
        let data = setup_data(&get_data("inputs/day02.txt").unwrap());
        let result = navigate(data);
        assert_eq!(result, 1_855_814);
    }

    #[test]
    fn task_2() {
        let data = setup_data(&get_data("inputs/day02.txt").unwrap());
        let result = navigate_aim(data);
        assert_eq!(result, 1_845_455_714);
    }
}
