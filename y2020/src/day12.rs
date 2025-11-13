use std::{ops::Neg, str::FromStr};

#[derive(Debug, Default, Clone, Copy)]
enum Direction {
    North,
    #[default]
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Right90(u16),
    Forward(u16),
    Direction(Direction, u16),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let action = chars.next().ok_or(())?;
        let number: u16 = chars.as_str().parse().map_err(|_| ())?;
        let action = match action {
            'N' => Self::Direction(Direction::North, number),
            'S' => Self::Direction(Direction::South, number),
            'E' => Self::Direction(Direction::East, number),
            'W' => Self::Direction(Direction::West, number),
            'L' => Self::Right90(4 - (number / 90)),
            'R' => Self::Right90(number / 90),
            'F' => Self::Forward(number),
            _ => return Err(()),
        };
        Ok(action)
    }
}

#[derive(Debug, Default)]
struct Location {
    x: i32,
    y: i32,
    facing: Direction,
}

impl Location {
    fn apply_action(&mut self, action: Action) {
        match action {
            Action::Right90(0) => {}
            Action::Right90(n) => {
                self.facing = match self.facing {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
                self.apply_action(Action::Right90(n - 1));
            }
            Action::Forward(n) => {
                let n = i32::from(n);
                match self.facing {
                    Direction::North => self.y -= n,
                    Direction::East => self.x += n,
                    Direction::South => self.y += n,
                    Direction::West => self.x -= n,
                }
            }
            Action::Direction(new_facing, n) => {
                let saved_facing = self.facing;
                self.facing = new_facing;
                self.apply_action(Action::Forward(n));
                self.facing = saved_facing;
            }
        }
    }

    const fn manhattan_from_center(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate_90_right(&mut self) {
        let tmp = self.x;
        self.x = self.y.neg();
        self.y = tmp;
    }

    fn apply_waypoint_action(&mut self, waypoint: &mut Self, action: Action) {
        match action {
            Action::Right90(n) => {
                for _ in 0..n {
                    waypoint.rotate_90_right();
                }
            }
            Action::Forward(n) => {
                let n = i32::from(n);
                self.x += waypoint.x * n;
                self.y += waypoint.y * n;
            }
            Action::Direction(direction, n) => {
                waypoint.apply_action(Action::Direction(direction, n));
            }
        }
    }
}

fn move_ship(input: &str) -> Result<i32, ()> {
    let mut ship: Location = Location::default();
    for line in input.lines() {
        let action = line.trim().parse()?;
        ship.apply_action(action);
    }
    Ok(ship.manhattan_from_center())
}

fn move_ship_by_waypoint(input: &str) -> Result<i32, ()> {
    let mut ship: Location = Location::default();
    let mut waypoint = Location { x: 10, y: -1, facing: Direction::default() };
    for line in input.lines() {
        let action = line.trim().parse()?;
        ship.apply_waypoint_action(&mut waypoint, action);
    }
    Ok(ship.manhattan_from_center())
}

#[advent_of_code_macros::aoc_tests]
mod tests {
    const EXAMPLE: &str = "F10
                           N3
                           F7
                           R90
                           F11";

    #[test]
    fn example_1() {
        assert_eq!(move_ship(EXAMPLE).unwrap(), 25);
    }

    #[test]
    fn part_1() {
        assert_eq!(move_ship(&read_input()).unwrap(), 1007);
    }

    #[test]
    fn example_2() {
        assert_eq!(move_ship_by_waypoint(EXAMPLE).unwrap(), 286);
    }

    #[test]
    fn part_2() {
        assert_eq!(move_ship_by_waypoint(&read_input()).unwrap(), 41212);
    }
}
