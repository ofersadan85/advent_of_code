use advent_of_code_common::grid::{Coords, Direction, Grid, GridCell};
use advent_of_code_macros::{aoc_tests, char_enum};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};
use tracing::debug;

#[char_enum(display)]
enum State {
    Wall = '#',
    Box = 'O',
    Robot = '@',
    Empty = '.',
}

#[char_enum(display)]
enum Expanded {
    Wall = '#',
    BoxLeft = '[',
    BoxRight = ']',
    Robot = '@',
    Empty = '.',
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

impl Gps for GridCell<State> {
    fn gps(&self) -> isize {
        match self.data {
            State::Box => self.x() + self.y() * 100,
            _ => 0,
        }
    }
}

impl Gps for GridCell<Expanded> {
    fn gps(&self) -> isize {
        match self.data {
            Expanded::BoxLeft => self.x() + self.y() * 100,
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
        .values()
        .find(|cell| cell.data == State::Robot)
        .ok_or_else(|| anyhow!("Robot not found"))?
        .as_point();
    let direction_cells = grid.sight_line(&robot, &direction, &[State::Wall, State::Empty]);
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
    match (first.data, last.data) {
        (_, State::Wall) | (State::Wall, _) => {} // Do nothing
        (_, State::Box) => return Err(anyhow!("Cannot push box past the edge")),
        (_, State::Robot) | (State::Robot, _) => return Err(anyhow!("Robot sees another robot")),
        (State::Empty, State::Empty) => {
            debug_assert_eq!(first, last); // We should only get to see "two" empty cells if they are the same cell
            grid.set(&robot, State::Empty);
            grid.set(&first.as_point(), State::Robot);
        }
        (State::Box, State::Empty) => {
            grid.set(&robot, State::Empty);
            grid.set(&first, State::Robot);
            grid.set(&last, State::Box);
        }
    }
    Ok(grid)
}

fn expand_grid(grid: &Grid<State>) -> Result<Grid<Expanded>> {
    let mut cells = BTreeMap::new();
    let x_range = grid.x_range.start..grid.x_range.end * 2;
    let y_range = grid.y_range.clone();
    for (index, cell) in grid.values().enumerate() {
        let index = isize::try_from(index).map_err(|e| anyhow!("Index out of range: {e}"))?;
        let new_states: [Expanded; 2] = cell.data.into();
        let x = ((index % x_range.end) * 2) % x_range.end;
        let p1 = (x, cell.y()).as_point();
        let p2 = (x + 1, cell.y()).as_point();
        cells.insert(p1, GridCell::new(&p1, new_states[0]));
        cells.insert(p2, GridCell::new(&p2, new_states[1]));
    }
    Ok(Grid {
        x_range,
        y_range,
        cells,
    })
}

fn push_step_expanded(mut grid: Grid<Expanded>, direction: Direction) -> Result<Grid<Expanded>> {
    let mut to_move = BTreeSet::new();
    let robot = grid
        .values()
        .find(|cell| cell.data == Expanded::Robot)
        .ok_or_else(|| anyhow!("Robot not found"))?
        .as_point();
    let mut to_visit = vec![robot];
    while let Some(cell) = to_visit.pop() {
        let next_cell = grid
            .neighbor_at(&cell, &direction)
            .ok_or_else(|| anyhow!("Next cell out of bounds"))?;
        if to_visit.contains(&next_cell.as_point()) {
            continue;
        }
        match next_cell.data {
            Expanded::Empty => {
                to_move.insert(cell);
            }
            Expanded::Wall => return Ok(grid),
            Expanded::BoxLeft => {
                to_move.insert(cell);
                to_visit.push(next_cell.as_point());
                let right = grid
                    .neighbor_at(next_cell, &Direction::East)
                    .ok_or_else(|| anyhow!("BoxLeft must have adjacent BoxRight"))?;
                debug_assert!(
                    right.data == Expanded::BoxRight,
                    "BoxLeft must have adjacent BoxRight: {right:?}"
                );
                if right.as_point() != cell.as_point() {
                    to_visit.push(right.as_point());
                }
            }
            Expanded::BoxRight => {
                to_move.insert(cell);
                to_visit.push(next_cell.as_point());
                let left = grid
                    .neighbor_at(next_cell, &Direction::West)
                    .ok_or_else(|| anyhow!("BoxRight must have adjacent BoxLeft"))?;
                debug_assert!(
                    left.data == Expanded::BoxLeft,
                    "BoxRight must have adjacent BoxLeft: {left:?}"
                );
                if left.as_point() != cell.as_point() {
                    to_visit.push(left.as_point());
                }
            }
            Expanded::Robot => return Err(anyhow!("Robot pushes another robot")),
        }
    }
    let mut to_move: Vec<_> = to_move.into_iter().collect();
    if Direction::South == direction || Direction::East == direction {
        to_move.reverse();
    }
    debug!(?to_move);
    for cell in to_move {
        let cell = grid
            .get(&cell)
            .ok_or_else(|| anyhow!("Cell not found"))?
            .clone();
        let next_cell = grid
            .neighbor_at(&cell, &direction)
            .ok_or_else(|| anyhow!("Next cell out of bounds"))?
            .clone();
        grid.set(&cell, Expanded::Empty);
        grid.set(&next_cell, cell.data);
    }
    println!("{grid}");
    Ok(grid)
}

fn grid_gps_values<T>(grid: &Grid<T>) -> isize
where
    GridCell<T>: Gps,
{
    grid.values().map(Gps::gps).sum()
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
