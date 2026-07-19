use crate::intcode::IntcodeComputer;
use advent_of_code_common::grid::{Direction, Grid, GridCell, Point};
use advent_of_code_macros::aoc_solver;
use std::str::FromStr;

struct Robot {
    computer: IntcodeComputer,
    position: Point,
    travel_distance: usize,
}

impl FromStr for Robot {
    type Err = <IntcodeComputer as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let computer: IntcodeComputer = s.parse()?;
        Ok(Self {
            computer,
            position: Point::default(),
            travel_distance: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Empty => write!(f, "."),
            Self::Oxygen => write!(f, "O"),
        }
    }
}

fn explore_grid(robot: Robot) -> Option<(Grid<Tile>, usize)> {
    let mut living_robots = vec![robot];
    let mut grid = Grid::new_dimensionless();
    let mut travel_distance = 0;
    while let Some(robot) = living_robots.pop() {
        for dir in Direction::orthogonal() {
            let mut new_robot = Robot {
                computer: robot.computer.clone(),
                position: robot.position + dir,
                travel_distance: robot.travel_distance + 1,
            };
            if grid.get(&new_robot.position).is_some() {
                continue;
            }
            let input_dir = match dir {
                Direction::North => 1_isize,
                Direction::South => 2,
                Direction::West => 3,
                Direction::East => 4,
                _ => unreachable!("Only orthogonal directions are allowed"),
            };
            new_robot.computer.queue_input(input_dir);
            new_robot.computer.run();
            match new_robot.computer.output.pop_front() {
                Some(0) => {
                    // Hit a wall, do nothing
                    let cell = GridCell::new(&new_robot.position, Tile::Wall);
                    grid.insert(new_robot.position, cell);
                }
                Some(1) => {
                    // Moved successfully, add new robot to living robots
                    let cell = GridCell::new(&new_robot.position, Tile::Empty);
                    grid.insert(new_robot.position, cell);
                    living_robots.push(new_robot);
                }
                Some(2) => {
                    // Found the oxygen system
                    let cell = GridCell::new(&new_robot.position, Tile::Oxygen);
                    grid.insert(new_robot.position, cell);
                    travel_distance = new_robot.travel_distance;
                }
                _ => return None, // Invalid output from the robot,
            }
        }
    }
    Some((grid.into(), travel_distance))
}

fn flood_fill_oxygen(mut grid: Grid<Tile>) -> usize {
    let mut changed = true;
    let mut minutes = 0;
    while changed {
        changed = false;
        let mut new_oxygen_positions = Vec::new();
        for cell in grid.cells.values().filter(|c| c.data == Tile::Oxygen) {
            for other in grid
                .neighbors_orthogonal(cell)
                .iter()
                .flatten()
                .filter(|c| c.data == Tile::Empty)
            {
                new_oxygen_positions.push(other.point);
            }
        }
        for pos in new_oxygen_positions {
            grid.insert(pos, GridCell::new(&pos, Tile::Oxygen));
            changed = true;
        }
        if changed {
            minutes += 1;
        }
    }
    minutes
}

#[aoc_solver(file = "inputs/2019/day15.txt", expected = 230)]
fn part_1(input: &str) -> usize {
    let robot: Robot = input.parse().expect("valid robot");
    let (_, travel_distance) = explore_grid(robot).expect("Oxygen system not found");
    travel_distance
}

#[aoc_solver(file = "inputs/2019/day15.txt", expected = 288)]
fn part_2(input: &str) -> usize {
    let robot: Robot = input.parse().expect("valid robot");
    let (grid, _) = explore_grid(robot).expect("Oxygen system not found");
    flood_fill_oxygen(grid)
}
