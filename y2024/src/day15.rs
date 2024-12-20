use advent_of_code_common::grid::{Direction, DxDy, Grid, PositionedCell};
use advent_of_code_macros::aoc_tests;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Wall,
    Box,
    Robot,
    Empty,
}

impl TryFrom<char> for State {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("Invalid state: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Expanded {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

impl From<State> for [Expanded; 2] {
    fn from(state: State) -> Self {
        match state {
            State::Wall => [Expanded::Wall; 2],
            State::Box => [Expanded::BoxLeft, Expanded::BoxRight],
            State::Robot => [Expanded::Robot, Expanded::Empty],
            State::Empty => [Expanded::Empty; 2],
        }
    }
}

trait Gps {
    fn gps(&self) -> isize;
}

impl Gps for PositionedCell<State> {
    fn gps(&self) -> isize {
        match self.state {
            State::Box => self.x + self.y * 100,
            _ => 0,
        }
    }
}

impl Gps for PositionedCell<Expanded> {
    fn gps(&self) -> isize {
        match self.state {
            Expanded::BoxLeft => self.x + self.y * 100,
            _ => 0,
        }
    }
}

fn parse_input(input: &str) -> Result<(Grid<State>, Vec<Direction>)> {
    let mut lines = input.lines();
    let grid = lines
        .take_while_ref(|line| !line.trim().is_empty())
        .join("\n")
        .parse()
        .map_err(|e| anyhow!("Failed to parse grid: {e}"))?;
    let instructions = lines
        .flat_map(|line| line.trim().chars())
        .filter_map(|c| Direction::try_from(c).ok())
        .collect();
    Ok((grid, instructions))
}

fn push_step(mut grid: Grid<State>, direction: Direction) -> Result<Grid<State>> {
    let robot = grid
        .cells
        .iter()
        .find(|cell| cell.state == State::Robot)
        .ok_or_else(|| anyhow!("Robot not found"))?;
    let direction_cells = grid.sight_line_cells(
        robot.x,
        robot.y,
        direction.dx(),
        direction.dy(),
        &[State::Wall, State::Empty],
    );
    let first = direction_cells
        .first()
        .map(Deref::deref)
        .ok_or_else(|| anyhow!("First seen cell must exist"))?
        .clone();
    let last = direction_cells
        .last()
        .map(Deref::deref)
        .ok_or_else(|| anyhow!("Last seen cell must exist"))?
        .clone();
    match (first.state, last.state) {
        (_, State::Wall) | (State::Wall, _) => {} // Do nothing
        (_, State::Box) => return Err(anyhow!("Cannot push box past the edge")),
        (_, State::Robot) | (State::Robot, _) => return Err(anyhow!("Robot sees another robot")),
        (State::Empty, State::Empty) => {
            debug_assert_eq!(first, last); // We should only get to see "two" empty cells if they are the same cell
            grid.set(robot.x, robot.y, State::Empty);
            grid.set(first.x, first.y, State::Robot);
        }
        (State::Box, State::Empty) => {
            grid.set(robot.x, robot.y, State::Empty);
            grid.set(first.x, first.y, State::Robot);
            grid.set(last.x, last.y, State::Box);
        }
    }
    Ok(grid)
}

fn expand_grid(grid: &Grid<State>) -> Result<Grid<Expanded>> {
    let mut cells = Vec::with_capacity(grid.cells.len() * 2);
    let x_range = grid.x_range.start..grid.x_range.end * 2;
    let y_range = grid.y_range.clone();
    for (index, cell) in grid.cells.iter().enumerate() {
        let index = isize::try_from(index).map_err(|e| anyhow!("Index out of range: {e}"))?;
        let new_states: [Expanded; 2] = cell.state.into();
        let x = ((index % x_range.end) * 2) % x_range.end;
        cells.push(PositionedCell::new(x, cell.y, new_states[0]));
        cells.push(PositionedCell::new(x + 1, cell.y, new_states[1]));
    }
    Ok(Grid {
        x_range,
        y_range,
        cells,
    })
}

fn push_step_expanded(mut grid: Grid<Expanded>, direction: Direction) -> Result<Grid<Expanded>> {
    let mut to_move = Vec::new();
    let robot = grid
        .cells
        .iter()
        .find(|cell| cell.state == Expanded::Robot)
        .ok_or_else(|| anyhow!("Robot not found"))?;
    let mut to_visit = vec![robot];
    while let Some(cell) = to_visit.pop() {
        let next_cell = grid
            .get_cell(cell.x + direction.dx(), cell.y + direction.dy())
            .ok_or_else(|| anyhow!("Next cell out of bounds"))?;
        if to_visit.contains(&next_cell) {
            continue;
        }
        match next_cell.state {
            Expanded::Empty => to_move.push(cell),
            Expanded::Wall => return Ok(grid),
            Expanded::BoxLeft => {
                to_move.push(cell);
                to_visit.push(next_cell);
                let right = grid
                    .get_cell(next_cell.x + 1, next_cell.y)
                    .ok_or_else(|| anyhow!("BoxLeft must have adjacent BoxRight"))?;
                debug_assert!(
                    right.state == Expanded::BoxRight,
                    "BoxLeft must have adjacent BoxRight"
                );
                if right != cell {
                    to_visit.push(right);
                }
            }
            Expanded::BoxRight => {
                to_move.push(cell);
                to_visit.push(next_cell);
                let left = grid
                    .get_cell(next_cell.x - 1, next_cell.y)
                    .ok_or_else(|| anyhow!("BoxRight must have adjacent BoxLeft"))?;
                debug_assert!(
                    left.state == Expanded::BoxLeft,
                    "BoxRight must have adjacent BoxLeft"
                );
                if left != cell {
                    to_visit.push(left);
                }
            }
            Expanded::Robot => return Err(anyhow!("Robot pushes another robot")),
        }
    }
    to_move = match direction {
        Direction::North | Direction::South => to_move.into_iter().sorted_by_key(|cell| cell.y),
        Direction::East | Direction::West => to_move.into_iter().sorted_by_key(|cell| cell.x),
        _ => return Err(anyhow!("Direction must be orthogonal")),
    }
    .collect();
    if Direction::South == direction || Direction::East == direction {
        to_move.reverse();
    }
    let to_move: Vec<_> = to_move.into_iter().cloned().collect();
    for cell in to_move {
        let next_cell = grid
            .get_cell(cell.x + direction.dx(), cell.y + direction.dy())
            .ok_or_else(|| anyhow!("Next cell out of bounds"))?
            .clone();
        grid.set(cell.x, cell.y, Expanded::Empty);
        grid.set(next_cell.x, next_cell.y, cell.state);
    }
    Ok(grid)
}

fn grid_gps_values<T>(grid: &Grid<T>) -> isize
where
    PositionedCell<T>: Gps,
{
    grid.cells.iter().map(Gps::gps).sum()
}

#[aoc_tests]
mod tests {
    #[test]
    fn example_1() {
        let input = std::fs::read_to_string("../inputs/2024/day15_example.txt").unwrap();
        let (grid, instructions) = parse_input(&input).unwrap();
        let result = instructions
            .iter()
            .try_fold(grid, |grid, &direction| push_step(grid, direction))
            .unwrap();
        assert_eq!(grid_gps_values(&result), 10092);
    }

    #[test]
    fn part_1() {
        let (grid, instructions) = parse_input(&read_input()).unwrap();
        let result = instructions
            .iter()
            .try_fold(grid, |grid, &direction| push_step(grid, direction))
            .unwrap();
        assert_eq!(grid_gps_values(&result), 1526673);
    }

    #[test]
    fn example_2() {
        let input = std::fs::read_to_string("../inputs/2024/day15_example.txt").unwrap();
        let (grid, instructions) = parse_input(&input).unwrap();
        let expanded = expand_grid(&grid).unwrap();
        let result = instructions
            .iter()
            .try_fold(expanded, |expanded, &direction| {
                push_step_expanded(expanded, direction)
            })
            .unwrap();
        assert_eq!(grid_gps_values(&result), 9021);
    }

    #[test]
    fn part_2() {
        let (grid, instructions) = parse_input(&read_input()).unwrap();
        let expanded = expand_grid(&grid).unwrap();
        let result = instructions
            .iter()
            .try_fold(expanded, |expanded, &direction| {
                push_step_expanded(expanded, direction)
            })
            .unwrap();
        assert_eq!(grid_gps_values(&result), 1535509);
    }
}
