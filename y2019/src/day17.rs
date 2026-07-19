use crate::intcode::{IntcodeComputer, State};
use advent_of_code_common::grid::{Direction, Grid, GridCell};
use advent_of_code_macros::aoc_solver;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Scaffold,
    Intersection,
    Robot(Direction),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Scaffold),
            '^' => Ok(Self::Robot(Direction::North)),
            'v' => Ok(Self::Robot(Direction::South)),
            '<' => Ok(Self::Robot(Direction::West)),
            '>' => Ok(Self::Robot(Direction::East)),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Scaffold => '#',
            Self::Intersection => 'O',
            Self::Robot(Direction::North) => '^',
            Self::Robot(Direction::South) => 'v',
            Self::Robot(Direction::West) => '<',
            Self::Robot(Direction::East) => '>',
            Self::Robot(_) => unreachable!("Invalid direction for robot"),
        };
        write!(f, "{c}")
    }
}

fn mark_intersections(grid: &mut Grid<Tile>) -> isize {
    let mut intersections = Vec::new();
    let mut value = 0;
    for cell in grid.values().filter(|c| matches!(c.data, Tile::Scaffold)) {
        if grid
            .neighbors_orthogonal(cell)
            .iter()
            .all(|c| c.is_some_and(|c| matches!(c.data, Tile::Scaffold)))
        {
            intersections.push(cell.point);
        }
    }
    for cell in intersections {
        if let Some(cell) = grid.get_mut(&cell) {
            cell.data = Tile::Intersection;
            value += cell.point.x * cell.point.y;
        }
    }
    value
}

struct Robot {
    computer: IntcodeComputer,
    grid: Grid<Tile>,
    extra_output: String,
}

fn ascii_from_output(computer: &mut IntcodeComputer) -> String {
    let mut output = String::new();
    while let Some(value) = computer.output.pop_front() {
        let value = u8::try_from(value).expect("valid ascii");
        let c = char::from(value);
        output.push(c);
    }
    output
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    TurnLeft,
    TurnRight,
    Forward(usize),
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TurnLeft => write!(f, "L"),
            Self::TurnRight => write!(f, "R"),
            Self::Forward(n) => write!(f, "{n}"),
        }
    }
}

impl Iterator for Robot {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        let robot = self.grid
            .values()
            .find(|c| matches!(c.data, Tile::Robot(_)))
            .expect("robot should exist");
        let robot_position = robot.point;
        let Tile::Robot(previous_direction) = robot.data else {
            unreachable!("robot should have a direction");
        };
        let left = previous_direction.turn_cw_270();
        let right = previous_direction.turn_cw_90();
        for direction in [previous_direction, left, right] {
            let next_cell = robot_position + direction;
            if matches!(
                self.grid.get(&next_cell),
                Some(&GridCell {
                    data: Tile::Scaffold,
                    ..
                })
            ) {
                match direction {
                    d if d == left => {
                        self.grid.get_mut(&robot_position).expect("checked").data = Tile::Robot(d);
                        return Some(Action::TurnLeft);
                    }
                    d if d == right => {
                        self.grid.get_mut(&robot_position).expect("checked").data = Tile::Robot(d);
                        return Some(Action::TurnRight);
                    }
                    d if d == previous_direction => {
                        self.grid.get_mut(&robot_position).expect("checked").data = Tile::Scaffold;
                        self.grid.get_mut(&next_cell).expect("checked").data = Tile::Robot(d);
                        return Some(Action::Forward(1));
                    }
                    _ => unreachable!("direction should be one of left, right, or previous"),
                }
            }
        }
        None
    }
}

impl FromStr for Robot {
    type Err = <Grid<Tile> as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut computer: IntcodeComputer = s.parse().expect("valid intcode");
        computer.memory.insert(0, 2); // Set to "wake up" the robot
        computer.run();
        let grid_output = ascii_from_output(&mut computer);
        let (grid_output, extra_output) = grid_output.split_once("\n\n").expect("valid output");
        Ok(Self {
            computer,
            grid: grid_output.trim_end_matches("\nMain:\n").parse()?,
            extra_output: extra_output.to_string(),
        })
    }
}

#[aoc_solver(file = "inputs/2019/day17.txt", expected = 12512)]
fn part_1(input: &str) -> isize {
    let mut robot: Robot = input.parse().expect("valid robot");
    mark_intersections(&mut robot.grid)
}

#[aoc_solver(file = "inputs/2019/day17.txt", expected = 0)]
fn part_2(input: &str) -> isize {
    let mut robot: Robot = input.parse().expect("valid robot");
    assert_eq!(robot.computer.state, State::AwaitingInput);
    for command in robot.by_ref() {
        println!("{command:?}");
    }
    println!("{}", robot.grid);
    todo!()
}
